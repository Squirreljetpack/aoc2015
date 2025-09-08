use std::collections::HashMap;

use good_lp::{solvers::coin_cbc::{CoinCbcProblem, CoinCbcSolution}, *};
use itertools::iproduct;
use ndarray::{Array2, ArrayView2};

advent_of_code::solution!(13);


pub fn parse(input: &str, metoo: bool) -> Option<ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>>> {
    let mut d: HashMap<u64, HashMap<u64, i32>> = HashMap::new();
    let mut count: u64 = 0;
    let mut atoi: HashMap<String, u64> = HashMap::new();

    for line in input.lines() {
        let (l, e) = line.trim_end_matches('.').split_once(" happiness units by sitting next to ")?;
        let (s, c) = l.split_once(" would ")?;
        let s_i = *atoi.entry(s.into()).or_insert_with(|| {
            count += 1;
            count - 1
        });
        let e_i = *atoi.entry(e.into()).or_insert_with(|| {
            count += 1;
            count - 1
        });
        let cost = if let Some(cost) = c.strip_prefix("gain ") {
            cost.parse::<i32>().ok()?
        } else {
            - c.strip_prefix("lose ")?.parse::<i32>().ok()?
        };
        d.entry(s_i)
            .or_default()
            .entry(e_i)
            .and_modify(|c| *c += cost)
            .or_insert( cost);
    }

    if metoo {
        let mut graph = Array2::<i32>::zeros(((count+1) as usize, (count+1) as usize));
        for (s, r) in d.into_iter() {
            for (e, c) in r.into_iter() {
                graph[[s as usize, e as usize]] += c;
                graph[[e as usize, s as usize]] += c;
            }
        }
        Some(graph)
    } else {
        let mut graph = Array2::<i32>::zeros((count as usize, count as usize));
        for (s, r) in d.into_iter() {
            for (e, c) in r.into_iter() {
                graph[[s as usize, e as usize]] += c;
                graph[[e as usize, s as usize]] += c;
            }
        }
        Some(graph)
    }


}

pub fn dot_vi(vars: ArrayView2<Variable>, costs: ArrayView2<i32>) -> Expression {
    let mut expr: Expression = 0.into();
    let n = vars.nrows();

    for i in 0..n {
        for j in 0..n {
            expr.add_mul(costs[[i, j]], vars[[i, j]]); // only scalar × IntoExpr
        }
    }

    // alternatively
    // let exprs: Array2<Expression> = adj.mapv(|v| v.into());
    // let obj: Expression = (exprs * &graph).fold(Expression::from(0), |acc, v| acc + v);

    expr
}

pub fn solve(graph: ArrayView2<i32>) -> u64 {
    let n = graph.nrows();
    let mut vars = ProblemVariables::new();
    let adj: Array2<Variable> = Array2::from_shape_fn((n, n), |_| vars.add(variable().binary()));

    let objective = dot_vi(adj.view(), graph);
    dbg!(graph);

    let mut problem = vars
        .maximise(&objective)
        .using(good_lp::default_solver);

    for i in 0..n {
        problem.add_constraint(constraint!(adj[[i,i]] <= 0));
        problem.add_constraint(constraint!(adj.row(i).fold(Expression::from(0), |acc, v| acc + v) == 1)); // one out
        problem.add_constraint(constraint!(adj.column(i).fold(Expression::from(0), |acc, v| acc + v) == 1)); // one in
    }
    for (i, j) in iproduct!(0..n, 0..n) {
        problem.add_constraint(constraint!(adj[[i,j]] + adj[[j, i]] <= 1)); // no doubling back
    }

    let mut solution = problem.clone().solve().unwrap();

    while ! is_tsp_solved(&mut problem, &solution, adj.view()) {
        solution = problem.clone().solve().unwrap();
    }

    solution.eval(&objective) as u64
}

pub fn is_tsp_solved(problem: &mut CoinCbcProblem, solution: &CoinCbcSolution, vars: ArrayView2<Variable>) -> bool {
    let n = vars.nrows();
    let mut cycle_idx: Vec<usize> = vec![0];
    let mut curr = 0;
    loop {
        let (max_idx, _) = vars.row(curr)
            .iter()
            .cloned()
            .enumerate()
            .max_by(|a, b| solution.value(a.1).partial_cmp(&solution.value(b.1)).unwrap())
            .unwrap();

        dbg!(solution.value(vars[[curr,max_idx]]));
        
        curr = max_idx;
        // every node belongs to a cycle
        if max_idx == cycle_idx[0] {
            break
        } else {
            cycle_idx.push(max_idx)
        }
    }
    let length = cycle_idx.len();
    dbg!(&cycle_idx);
    dbg!(length);
    if length < n {
        let expr = iproduct!(cycle_idx.iter(), cycle_idx.iter())
            .map(|(&i, &j)| vars[[i as usize, j as usize]].into())
            .fold(Expression::from(0), |acc, v: Expression| acc + v);

        problem.add_constraint(expr.leq((cycle_idx.len() as u32) - 1));

        false
    } else {
        true
    }
    
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse(input, false)?;
    let result = solve(graph.view());
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse(input, true)?;
    let result = solve(graph.view());
    Some(result)
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
