use std::fmt;
use std::slice;
use node::Term;


#[derive(PartialEq, Debug, Clone)]
/// An Expression is comprised of any number of Terms
pub struct Expression {
    terms: Vec<Term>,
}

impl Expression {
    pub fn new() -> Expression {
        Expression { terms: vec![] }
    }

    pub fn from_parts(v: Vec<Term>) -> Expression {
        Expression { terms: v }
    }

    pub fn terms_iter(&self) -> Iter {
        Iter {
            iterator: self.terms.iter(),
        }
    }

    pub fn add_term(&mut self, term: Term) {
        self.terms.push(term)
    }

    pub fn remove_term(&mut self, term: &Term) -> Option<Term> {
        if let Some(pos) = self.terms.iter().position(|x| *x == *term) {
            Some(self.terms.remove(pos))
        } else {
            None
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = self.terms
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "{}", display)
    }
}

pub struct Iter<'a> {
    iterator: slice::Iter<'a, Term>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Term;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_expressions() {
        let t1: Term = Term::Terminal(String::from("terminal"));
        let nt1: Term = Term::Nonterminal(String::from("nonterminal"));
        let t2: Term = Term::Terminal(String::from("terminal"));
        let nt2: Term = Term::Nonterminal(String::from("nonterminal"));

        let e1: Expression = Expression::from_parts(vec![nt1, t1]);
        let mut e2: Expression = Expression::new();
        e2.add_term(nt2);
        e2.add_term(t2);

        assert_eq!(e1, e2);
    }

    #[test]
    fn add_term_to_expression() {
        let mut terms = vec![
            Term::Terminal(String::from("A")),
            Term::Terminal(String::from("C")),
            Term::Terminal(String::from("G")),
        ];

        let mut dna_expression = Expression::from_parts(terms.clone());
        assert_eq!(dna_expression.terms_iter().count(), terms.len());

        // oops forgot "T"
        let forgotten = Term::Terminal(String::from("T"));
        dna_expression.add_term(forgotten.clone());
        terms.push(forgotten);
        assert_eq!(dna_expression.terms_iter().count(), terms.len());

        // check all terms are there
        for term in dna_expression.terms_iter() {
            assert!(terms.contains(term), "{} was not in terms", term);
        }
    }

    #[test]
    fn remove_term_from_expression() {
        let terms = vec![
            Term::Terminal(String::from("A")),
            Term::Terminal(String::from("C")),
            Term::Terminal(String::from("G")),
            Term::Terminal(String::from("T")),
            Term::Terminal(String::from("Z")),
        ];

        let mut dna_expression = Expression::from_parts(terms.clone());
        assert_eq!(dna_expression.terms_iter().count(), terms.len());

        // oops "Z" isn't a dna base
        let accident = Term::Terminal(String::from("Z"));
        let removed = dna_expression.remove_term(&accident);

        // the removed element should be the accident
        assert_eq!(Some(accident.clone()), removed);
        // number of terms should have decreased
        assert_eq!(dna_expression.terms_iter().count(), terms.len() - 1);
        // the accident should no longer be found in the terms
        assert_eq!(
            dna_expression.terms_iter().find(|&term| *term == accident),
            None
        );
    }

    #[test]
    fn remove_nonexistent_term_from_expression() {
        let terms = vec![
            Term::Terminal(String::from("A")),
            Term::Terminal(String::from("C")),
            Term::Terminal(String::from("G")),
            Term::Terminal(String::from("T")),
        ];

        let mut dna_expression = Expression::from_parts(terms.clone());
        assert_eq!(dna_expression.terms_iter().count(), terms.len());

        // oops "Z" isn't a dna base
        let nonexistent = Term::Terminal(String::from("Z"));
        let removed = dna_expression.remove_term(&nonexistent);

        // the removed element should be the accident
        assert_eq!(None, removed);
        // number of terms should not have decreased
        assert_eq!(dna_expression.terms_iter().count(), terms.len());
    }
}
