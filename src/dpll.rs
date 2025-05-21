use crate::parser::CNF;
use std::collections::{BTreeSet, HashSet};
// TODO: This is quite Object oriented atm. Could make more rustish
impl CNF {
    // TODO: Use Queue to make more efficient
    pub fn unit_propergate(&mut self) -> bool {
        let units = self
            .clauses
            .iter()
            .filter(|cl| cl.len() == 1)
            .map(|cl| *cl.iter().next().unwrap())
            .collect::<Vec<_>>();
        if units.iter().any(|u| units.contains(&-u)) {
            return false;
        }

        if units.is_empty() {
            return true;
        }

        self.cleanse_true_literals(units);
        self.unit_propergate()
    }

    fn cleanse_true_literals(&mut self, truths: Vec<i32>) {
        if truths.is_empty() {
            return;
        }
        self.clauses
            .retain(|clause| !truths.iter().any(|&unit| clause.contains(&unit)));
        let old_clauses = std::mem::take(&mut self.clauses);
        self.clauses = old_clauses
            .into_iter()
            .map(|clause| {
                clause
                    .into_iter()
                    .filter(|&lit| !truths.contains(&-lit))
                    .collect::<BTreeSet<i32>>()
            })
            .collect();
    }

    fn choose_literal(&self) -> i32 {
        self.clauses.iter().flatten().next().copied().unwrap()
    }

    pub fn purify_literals(&mut self) {
        let literals: HashSet<i32> = self.clauses.iter().flatten().copied().collect();
        let pure_literals: Vec<i32> = literals
            .iter()
            .filter(|&&lit| !literals.contains(&-lit))
            .copied()
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect();

        self.cleanse_true_literals(pure_literals);
    }

    pub fn dpll(&mut self) -> bool {
        if !self.unit_propergate() {
            return false;
        }
        self.purify_literals();
        if self.clauses.is_empty() {
            true
        } else if self.clauses.iter().any(|clause| clause.is_empty()) {
            false
        } else {
            let literal = self.choose_literal();

            let mut branch = self.clone();
            branch.cleanse_true_literals(vec![literal]);
            if branch.dpll() {
                return true;
            }

            let mut branch = self.clone();
            branch.cleanse_true_literals(vec![-literal]);
            if branch.dpll() {
                return true;
            }
            false
        }
    }
}
