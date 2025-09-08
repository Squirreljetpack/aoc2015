#![allow(unused)]
#![allow(non_snake_case)]

use core::f64;
use std::ops::RangeInclusive;

use anyhow::{anyhow, bail};
use clarabel::{algebra::CscMatrix, solver::{DefaultSettings, DefaultSolver, IPSolver, SupportedConeT}};
use good_lp::constraint;
use itertools::Itertools;
use nalgebra::{DMatrix, DMatrixView, DVector, RowDVector};
use argmin::{
    core::{observers::ObserverMode, CostFunction, Error, Executor, Gradient, Hessian, State},
    solver::{linesearch::MoreThuenteLineSearch, newton::NewtonCG},
};
use argmin_observer_slog::SlogLogger;
use argmin_math::*;

advent_of_code::solution!(15);

macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!($($arg)*);
        }
    };
}

pub fn parse(input: &str) -> (DMatrix<f64>, RowDVector<f64>) {
    let count = input.lines().count();
    
    let mut props = Vec::with_capacity(count * 4);
    let mut calories = Vec::with_capacity(count);
    
    for line in input.lines() {
        let rest = line.split_once(':').unwrap().1.trim();
        let mut numbers: Vec<f64> = rest
        .split_whitespace()
        .filter_map(|s| s.trim_end_matches(',').parse::<f64>().ok())
        .collect();
        
        let cal = numbers.pop().unwrap();
        calories.push(cal);
        
        props.extend(numbers);
    }
    
    // Each ingredient in a column
    let a = DMatrix::from_iterator(4, count, props.into_iter());
    let cal_vec = RowDVector::from_iterator(count, calories.into_iter());
    
    (a, cal_vec)
}

pub struct Triplets<T> {
    pub colranges: Vec<usize>, // (r, v) in [i]..[i+1] specify A_ri = v
    pub rowval: Vec<usize>,
    pub nzval: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

pub fn dmatrix_to_triplets(dm: &DMatrix<f64>) -> Triplets<f64> {
    let (rows, cols) = dm.shape();
    let mut colranges = Vec::with_capacity(cols + 1);
    let mut rowval = Vec::new();
    let mut nzval = Vec::new();
    
    colranges.push(0);
    let mut range_end = 0;
    for j in 0..cols {
        for i in 0..rows {
            let v = dm[(i, j)];
            if v != 0.0 {
                rowval.push(i);
                nzval.push(v);
                range_end += 1;
            }
        }
        colranges.push(range_end);
    }
    
    Triplets { colranges, rowval, nzval, rows, cols }
}

pub fn dmatrix_to_csc(dm: &DMatrix<f64>) -> CscMatrix<f64> {
    let t = dmatrix_to_triplets(&dm);
    CscMatrix::new(
        t.rows,
        t.cols,
        t.colranges,
        t.rowval,
        t.nzval,
    )
}

pub fn find_feasible(A: CscMatrix, b: &[f64], cones: &[SupportedConeT<f64>]) -> anyhow::Result<Vec<f64>> {
    
    // dbg!(&A, &b, &cones);
    let P = CscMatrix::zeros((A.n, A.n));  // no quadratic term
    let q = vec![0.0; A.n];         // no linear term
    
    let settings = DefaultSettings::default();
    let mut solver = DefaultSolver::new(&P, &q, &A, &b, &cones, settings)?;
    
    solver.solve();
    
    debug_eprintln!("Feasible (x) = {:?}", solver.solution.x);
    
    Ok(solver.solution.x.clone())
}

// row major storage so we need to allocate
pub fn choose_columns(a: &DMatrix<f64>, selected: &[usize]) -> DMatrix<f64> {
    DMatrix::from_columns(
        &selected.iter().map(|&i| a.column(i)).collect::<Vec<_>>()
    )
}

pub fn split_columns(a: &DMatrix<f64>, left_split: &[usize]) -> (DMatrix<f64>, DMatrix<f64>) {
    let ncols = a.ncols();
    
    // Compute complement indices
    let complement: Vec<usize> = (0..ncols)
    .filter(|i| !left_split.contains(i))
    .collect();
    
    (choose_columns(a, left_split), choose_columns(a, &complement))
}

pub fn find_invertible_partition(
    m: &DMatrix<f64>,
    k: usize
) -> anyhow::Result<(DMatrix<f64>, DMatrix<f64>, DMatrix<f64>, Vec<usize>)> {
    let n_cols = m.ncols();
    if k > n_cols {
        bail!("k cannot be greater than the number of columns");
    }
    
    for left_split in (0..n_cols).combinations(k) {
        let (m_e, m_r) = split_columns(m, &left_split);
        
        if let Some(inv) = m_e.clone().try_inverse() {
            return Ok((m_e, m_r, inv, left_split));
        }
    }
    
    bail!("Could not find a valid permutation of columns such that m_e is invertible")
}


pub fn reduce_with_constraints(
    a: &DMatrix<f64>,
    constraints_matrix: &DMatrix<f64>,
    c_vec: &DVector<f64>,
) -> Result<(DMatrix<f64>, DVector<f64>, DMatrix<f64>, DVector<f64>, Vec<usize>), Error> {
    let (m_dim, n_dim) = a.shape();
    let (k, n_constraints) = constraints_matrix.shape();
    
    // Check dimensions
    if k == 0 {
        // No constraints
        let zero_matrix = DMatrix::zeros(a.nrows(), 0);
        let zero_vector = DVector::zeros(0);
        
        return Ok((a.clone(), DVector::zeros(m_dim), zero_matrix, zero_vector, vec![]));
    }
    if n_dim < k {
        bail!("More constraints ({}) than variables ({}).", k, n_dim);
    }
    if n_constraints != n_dim {
        bail!("Dimension mismatch: A.ncols ({}) must equal constraints_matrix.ncols ({}).", n_dim, n_constraints);
    }
    if c_vec.nrows() != k {
        bail!("Dimension mismatch: constraints_matrix.nrows ({}) must equal c_vec.nrows ({}).", k, c_vec.nrows());
    }
    
    // Partition A into A_R (retained) and A_E (eliminated)
    let (m_e, m_r, m_e_inv, left_split) = find_invertible_partition(constraints_matrix, k)?;
    let (a_e, a_r) = split_columns(a, &left_split);
    
    
    // y = A_R x_R - A_E * M_E_inv * M_R * x_r + A_E * M_E_inv * c
    
    let xe_linear = - &m_e_inv * &m_r;
    let xe_affine = &m_e_inv * c_vec;
    
    let b_prime = &a_e * &xe_affine;
    
    let a_prime = &a_r + &a_e * &xe_linear;
    
    Ok((a_prime, b_prime, xe_linear, xe_affine, left_split))
}

const T: f64 = 100.0;

fn obj(a: &DMatrix<f64>, b: &DVector<f64>, p: &DVector<f64>) -> Result<f64, Error> {
    let v = a * p + b;
    
    // positive amounts + coefficients + bounded total
    if v.iter().any(|&v| v <= 0.0) ||
    p.iter().any(|&v| v <= 0.0) ||
    p.sum() >= T 
    {
        bail!("Infeasible obj: {a}, {b}, {p}")
    }
    let obj = -v.map(|v| v.ln()).sum();
    Ok(obj)
}

// use on full matrix, computes the actual answer
fn obj_int(a: &DMatrix<i64>, p: &DVector<i64>) -> Result<f64, Error> {
    // Compute a * p
    let v = a * p;
    
    if v.iter().any(|&v| v <= 0) ||
    p.iter().any(|&v| v <= 0) ||
    p.sum() != T as i64
    {
        bail!("Infeasible obj v: {v}, p: {p}, {}", p.sum())
    }
    
    let obj: i64 = v.iter().product();
    
    debug_eprintln!("A: {a}, p: {p}, v: {v}, obj: {obj}");
    Ok(obj as f64)
}

fn obj_int_calories(a: &DMatrix<i64>, calories: &RowDVector<i64>, p: &DVector<i64>) -> Result<f64, Error> {
    // Compute a * p
    let v = a * p;

    let sum = p.sum();
    let cals = (calories * p)[0];
    
    if v.iter().any(|&v| v <= 0) ||
    p.iter().any(|&v| v <= 0) ||
    sum != T as i64 ||
    cals != 500
    {
        bail!("Infeasible obj v: {v}, p: {p}, {}, {}", sum, cals)
    }
    
    // log of product
    let obj: i64 = v.iter().product();
    
    #[cfg(debug_assertions)]
    {
        debug_eprintln!("A: {a}, p: {p}, v: {v}, obj: {obj}");
    }
    Ok(obj as f64)
}

struct OptProblem {
    a: DMatrix<f64>,
    b: DVector<f64>,
    c: f64, // Barrier parameter
}

impl CostFunction for OptProblem {
    type Param = DVector<f64>;
    type Output = f64;
    
    fn cost(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        let v = obj(&self.a, &self.b, &p).unwrap_or(f64::INFINITY);
        
        let barrier = -self.c * (p.map(|v| v.ln()).sum() + (T - p.sum()).ln());
        
        Ok(v + barrier)
    }
}

impl Gradient for OptProblem {
    type Param = DVector<f64>;
    type Gradient = DVector<f64>;
    
    fn gradient(&self, p: &Self::Param) -> Result<Self::Gradient, Error> {
        let v = &self.a * p + &self.b;
        
        if v.iter().any(|&v| v <= 0.0) ||
        p.iter().any(|&v| v <= 0.0) ||
        p.sum() >= T
        {
            anyhow::bail!("Gradient calculation in infeasible region")
        }
        
        let iv = v.map(|val| 1.0 / val);
        // f'_j = Σ_i 1/v_i(∂v_i/∂x_j)  = -A^T v^-1
        let grad = -self.a.transpose() * iv;
        // = -c (∂Σ1/xi)/∂x_j = -c (1/x_j)
        let grad_barrier_p = -self.c * p.map(|vj| 1.0 / vj);
        // = - c (∂ln(T-Σp_i))/∂p_j = 
        let grad_barrier_sum_scalar = self.c / (T - p.sum());
        let grad_barrier_sum = DVector::from_element(p.len(), grad_barrier_sum_scalar);
        
        Ok(grad + grad_barrier_p + grad_barrier_sum)
    }
}

impl Hessian for OptProblem {
    type Param = DVector<f64>;
    type Hessian = DMatrix<f64>;
    
    fn hessian(&self, p: &Self::Param) -> Result<Self::Hessian, Error> {
        let v = &self.a * p + &self.b;
        let n = p.len();
        
        if v.iter().any(|&v| v <= 0.0) ||
        p.iter().any(|&v| v <= 0.0) ||
        p.sum() >= T
        {
            anyhow::bail!("Hessian calculation in infeasible region");
        }
        
        // 1. Hessian is A^T * D * A
        // where D is a diagonal matrix with D_ii = 1 / (Ap)_i^2
        let d_diag = v.map(|v| 1.0 / v.powi(2));
        let d_mat = DMatrix::from_diagonal(&d_diag);
        let h1 = self.a.transpose() * d_mat * &self.a;
        
        // 2. grad -c/x_j = D_ii = c / p_i^2
        let h2_diag = p.map(|v| self.c / v.powi(2));
        let h2 = DMatrix::from_diagonal(&h2_diag);
        
        // 3. grad (c/T-Σp) = - c / (T - Σp)^2 (∂T-Σp)/∂p_i =
        // constant c / (T - Σp)^2
        let h3_scalar = self.c / (T - p.sum()).powi(2);
        let h3 = DMatrix::from_element(n, n, h3_scalar);
        
        Ok(h1 + h2 + h3)
    }
}

pub fn solve(a: &DMatrix<f64>, c: f64) -> Result<(Vec<i64>, f64), Error> {
    let m = a.nrows();
    let mut n = a.ncols(); // param count
    let a_orig = a.map(|v| v as i64);
    let (a, a_rem, xe_linear, xe_affine, left_split) = reduce_with_constraints(&a, &DMatrix::from_row_slice(1, n, &vec![1.0; n]), &DVector::from_element(1, 100.0))?;
    n = n - 1;
    debug_eprintln!("Transformed matrix A:\n{}\n, a_rem\n{}\n, xe_linear:\n{}\n, xe_affine:\n{}", a, a_rem, xe_linear, xe_affine);
    
    let constraint_a = a.clone().map(|v| -v);
    let identity = DMatrix::identity(n, n);
    let sum_row = &DMatrix::<f64>::from_row_slice(1, n, &vec![1.0; n]);
    
    // outputs positive + inputs positive + sum <= 100
    let A_stacked = DMatrix::from_rows(
        &constraint_a
        .row_iter()
        .chain(identity.map(|v: f64| -v).row_iter())
        .chain(sum_row.row_iter())
        .collect::<Vec<_>>()
    );
    
    // b vector
    let b_stacked = DVector::from_iterator(
        m + n + 1,
        a_rem.iter().copied().map(|v| v -1.0)
        .chain(vec![0.0; n].into_iter()) // Ax >= 1, x >= 1
        .chain(Some(100.0))
    );
    
    let mut cones = Vec::new();
    cones.push(SupportedConeT::NonnegativeConeT(m + n + 1));
    
    debug_eprintln!("Initial constraints Ax+s=b:\n{}\n{:?}\n{}", A_stacked, cones, b_stacked);
    
    let initial = find_feasible(dmatrix_to_csc(&A_stacked), b_stacked.as_slice(), &cones)?;
    let initial_obj = obj(&a, &a_rem, &DVector::from_vec(initial.clone()))?;
    debug_eprintln!("Initial objective: {initial_obj}");
    
    // cost fn
    let problem = OptProblem { a: a.clone(), b: a_rem.clone(), c };
    
    // set up line search
    let linesearch = MoreThuenteLineSearch::new();
    
    // Set up solver
    let solver = NewtonCG::new(linesearch);
    
    // Run solver
    let res = Executor::new(problem, solver)
    .configure(|state| state.param(DVector::from_vec(initial)).max_iters(100))
    .add_observer(SlogLogger::term(), ObserverMode::Always)
    .run()?;
    
    // Print result
    debug_eprintln!("{res}");
    
    // why doesn't bail work here in place of anyhow?
    let relaxed_best = res.state().get_best_param().ok_or_else(|| anyhow!("missing val"))?;
    
    let recovered_relaxed_best = recover_full_vec(relaxed_best, &xe_linear, &xe_affine, &left_split);
    
    debug_eprintln!("relaxed full: {recovered_relaxed_best}");
    
    // ok because convex
    let neighborhood_best = fuzz_int_max(&recovered_relaxed_best,
        0,
        |p| {
            obj_int(&a_orig, &DVector::from_column_slice(p))
        }
    ).unwrap();
        
    Ok(neighborhood_best)
}

fn recover_full_vec(
    p: &DVector<f64>,
    xe_linear: &DMatrix<f64>,
    xe_affine: &DVector<f64>,
    left_split: &[usize],
    ) -> DVector<f64> {
    let x_e = xe_linear * p + xe_affine;

    // allocate full vector
    let mut full = DVector::zeros(p.len() + left_split.len());
    let mut p_iter = p.iter();

    for i in 0..full.len() {
        if let Some(pos) = left_split.iter().position(|&idx| idx == i) {
            full[i] = x_e[pos];
        } else {
            full[i] = *p_iter.next().unwrap();
        }
    }

    full
}

pub fn solve2(a: &DMatrix<f64>, calories: &RowDVector<f64>, c: f64) -> Result<(Vec<i64>, f64), Error> {
    let m = a.nrows();
    let mut n = a.ncols(); // param count
    let a_orig = a.map(|v| v as i64);

    // Stack rows into a DMatrix
    let constraints = DMatrix::from_rows(
        &[
            RowDVector::from_element(n, 1.0),
            calories.clone()
        ]
    );
    
    
    let (a, a_rem, xe_linear, xe_affine, left_split) = reduce_with_constraints(&a, &constraints, &DVector::from_vec(vec![100.0, 500.0]))?;
    
    n = n - 2;
    debug_eprintln!("Transformed matrix A:\n{}\n, a_rem\n{}\n, xe_linear:\n{}\n, xe_affine:\n{}", a, a_rem, xe_linear, xe_affine);
    
    let constraint_a = a.clone().map(|v| -v);
    let identity = DMatrix::identity(n, n);
    let sum_row = &DMatrix::<f64>::from_row_slice(1, n, &vec![1.0; n]);
    
    // Stack vertically
    let A_stacked = DMatrix::from_rows(
        &constraint_a
        .row_iter()
        .chain(identity.map(|v: f64| -v).row_iter())
        .chain(sum_row.row_iter())
        .collect::<Vec<_>>()
    );
    
    // b vector
    let b_stacked = DVector::from_iterator(
        m + n + 1,
        a_rem.iter().copied().map(|v| v -1.0)
        .chain(vec![0.0; n].into_iter()) // Ax >= 1, x >= 1
        .chain(Some(100.0))
    );
    
    let mut cones = Vec::new();
    cones.push(SupportedConeT::NonnegativeConeT(m + n + 1));
    
    debug_eprintln!("Initial constraints Ax+s=b:\n{}\n{:?}\n{}", A_stacked, cones, b_stacked);
    
    let initial = find_feasible(dmatrix_to_csc(&A_stacked), b_stacked.as_slice(), &cones)?;
    let initial_obj = obj(&a, &a_rem, &DVector::from_vec(initial.clone()))?;
    debug_eprintln!("Initial objective: {initial_obj}, {}", (-initial_obj).exp());
    
    
    // cost fn
    let problem = OptProblem { a: a.clone(), b: a_rem.clone(), c };
    
    // set up line search
    let linesearch = MoreThuenteLineSearch::new();
    
    // Set up solver
    let solver = NewtonCG::new(linesearch);
    
    // Run solver
    let res = Executor::new(problem, solver)
    .configure(|state| state.param(DVector::from_vec(initial)).max_iters(100))
    .add_observer(SlogLogger::term(), ObserverMode::Always)
    .run()?;
    
    // Print result
    debug_eprintln!("{res}");
    
    // why doesn't bail work here in place of anyhow?
    let relaxed_best = res.state().get_best_param().ok_or_else(|| anyhow!("missing val"))?;
    
    // could be improved by fuzzing over the reduced vector but its cleaner this way
    let recovered_relaxed_best = recover_full_vec(relaxed_best, &xe_linear, &xe_affine, &left_split);
    
    debug_eprintln!("relaxed full: {recovered_relaxed_best}");
    
    let calories_int = calories.map(|v| v as i64);
    // ok because convex
    let neighborhood_best = fuzz_int_max(&recovered_relaxed_best,
        2,
        |p| {
            obj_int_calories(&a_orig, &calories_int, &DVector::from_column_slice(p))
        }
    ).unwrap();
        
    Ok(neighborhood_best)
}

// todo: other strategies also possible...
pub fn fuzz_int_max<F>(center: &DVector<f64>, radius: u32, obj: F) -> Result<(Vec<i64>, f64), Error>
where 
F: Fn(&[i64]) -> Result<f64, Error>,
{
    let mut best_val = -1.0;
    let mut best_vec = None;
    let mut count = 0;
    
    // inconstant width but not a problem
    let ranges: Vec<RangeInclusive<i64>> = center
    .iter()
    .map(|&c| (c.floor() as i64 - radius as i64)..=(c.ceil() as i64 + radius as i64))
    .collect();
    
    for v in ranges.into_iter().multi_cartesian_product() {
        let curr = obj(&v);
        match curr {
            Ok(curr) => {
                count = count + 1;
                if curr > best_val {
                    best_val = curr;
                    best_vec = Some(v)
                }
            }
            Err(e) => {
                debug_eprintln!("Skipped: {e}")
            }
        }
    }

    if let Some(best_vec) = best_vec {
        debug_eprintln!("Checked {count} feasible solutions");
        return Ok((best_vec, best_val))
    } else {
        bail!("Empty range")
    }
}



pub fn part_one(input: &str) -> Option<u64> {
    let (a, _) = parse(input);
    let barrier_param = 1e-3;
    
    debug_eprintln!("Solving for matrix A:\n{}\n with barrier c = {}", a, barrier_param);
    debug_eprintln!("-----------------------------------------------");
    
    let mut best = solve(&a, barrier_param).unwrap_or_else(|e| panic!("Solver failed: {}", e));
    
    debug_eprintln!("Best: {:?}, {}", best.0, best.1);
    
    Some(best.1 as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (a, calories) = parse(input);
    let barrier_param = 1e-3;
    
    debug_eprintln!("Solving for matrix A:\n{}\n with calories {}, barrier c = {}", a, calories, barrier_param);
    debug_eprintln!("-----------------------------------------------");
    
    let mut best = solve2(&a, &calories, barrier_param).unwrap_or_else(|e| panic!("Solver failed: {}", e));

    let a = a.map(|x| x as i64);
    let calories = calories.clone().map(|x| x as i64);
    
    debug_eprintln!("Best: {:?}, Counts: {}, Calories: {}, {}", best.0, a * DVector::from_column_slice(&best.0), calories * DVector::from_column_slice(&best.0), best.1);
    
    Some(best.1 as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
