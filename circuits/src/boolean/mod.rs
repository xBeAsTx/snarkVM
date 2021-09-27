// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

pub mod not;
pub use not::*;

use crate::{traits::*, Environment, LinearCombination, Mode, Variable};
use snarkvm_fields::{One as O, Zero as Z};

use std::ops::Not;

#[derive(Clone)]
pub struct Boolean<E: Environment>(LinearCombination<E::Field>);

impl<E: Environment> Boolean<E> {
    pub fn new(mode: Mode, value: bool) -> Self {
        let variable = E::new_variable(mode, match value {
            true => E::Field::one(),
            false => E::Field::zero(),
        });

        if !mode.is_constant() {
            // Ensure `a` is either 0 or 1:
            // (1 - a) * a = 0
            E::enforce(|| (E::one() - variable, variable, E::zero()));
        }

        Self(variable.into())
    }

    pub fn to_value(&self) -> bool {
        let value = self.0.to_value();
        debug_assert!(value.is_zero() || value.is_one());
        value.is_one()
    }
}

impl<E: Environment> BooleanTrait for Boolean<E> {}

impl<E: Environment> From<Boolean<E>> for LinearCombination<E::Field> {
    fn from(boolean: Boolean<E>) -> Self {
        boolean.0
    }
}

impl<E: Environment> From<&Boolean<E>> for LinearCombination<E::Field> {
    fn from(boolean: &Boolean<E>) -> Self {
        boolean.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Circuit;

    #[test]
    fn test_new_constant() {
        assert_eq!(0, Circuit::num_constants());
        assert_eq!(1, Circuit::num_public());
        assert_eq!(0, Circuit::num_private());
        assert_eq!(0, Circuit::num_constraints());

        let candidate = Boolean::<Circuit>::new(Mode::Constant, false);
        assert_eq!(false, candidate.to_value());
        assert!(Circuit::is_satisfied());

        let candidate = Boolean::<Circuit>::new(Mode::Constant, true);
        assert_eq!(true, candidate.to_value());
        assert!(Circuit::is_satisfied());

        assert_eq!(2, Circuit::num_constants());
        assert_eq!(1, Circuit::num_public());
        assert_eq!(0, Circuit::num_private());
        assert_eq!(0, Circuit::num_constraints());
    }

    #[test]
    fn test_new_public() {
        assert_eq!(0, Circuit::num_constants());
        assert_eq!(1, Circuit::num_public());
        assert_eq!(0, Circuit::num_private());
        assert_eq!(0, Circuit::num_constraints());

        let candidate = Boolean::<Circuit>::new(Mode::Public, false);
        assert_eq!(false, candidate.to_value());
        assert!(Circuit::is_satisfied());

        let candidate = Boolean::<Circuit>::new(Mode::Public, true);
        assert_eq!(true, candidate.to_value());
        assert!(Circuit::is_satisfied());

        assert_eq!(0, Circuit::num_constants());
        assert_eq!(3, Circuit::num_public());
        assert_eq!(0, Circuit::num_private());
        assert_eq!(2, Circuit::num_constraints());
    }

    #[test]
    fn test_new_private() {
        assert_eq!(0, Circuit::num_constants());
        assert_eq!(1, Circuit::num_public());
        assert_eq!(0, Circuit::num_private());
        assert_eq!(0, Circuit::num_constraints());

        let candidate = Boolean::<Circuit>::new(Mode::Private, false);
        assert_eq!(false, candidate.to_value());
        assert!(Circuit::is_satisfied());

        let candidate = Boolean::<Circuit>::new(Mode::Private, true);
        assert_eq!(true, candidate.to_value());
        assert!(Circuit::is_satisfied());

        assert_eq!(0, Circuit::num_constants());
        assert_eq!(1, Circuit::num_public());
        assert_eq!(2, Circuit::num_private());
        assert_eq!(2, Circuit::num_constraints());
    }

    #[test]
    fn test_new_fail() {
        let one = <Circuit as Environment>::Field::one();
        let two = one + one;
        {
            let candidate = Circuit::new_variable(Mode::Constant, two);

            // Ensure `a` is either 0 or 1:
            // (1 - a) * a = 0
            Circuit::enforce(|| (Circuit::one() - candidate, candidate, Circuit::zero()));
            assert!(!Circuit::is_satisfied());

            Circuit::reset_circuit();
        }
        {
            let candidate = Circuit::new_variable(Mode::Public, two);

            // Ensure `a` is either 0 or 1:
            // (1 - a) * a = 0
            Circuit::enforce(|| (Circuit::one() - candidate, candidate, Circuit::zero()));
            assert!(!Circuit::is_satisfied());

            Circuit::reset_circuit();
        }
        {
            let candidate = Circuit::new_variable(Mode::Private, two);

            // Ensure `a` is either 0 or 1:
            // (1 - a) * a = 0
            Circuit::enforce(|| (Circuit::one() - candidate, candidate, Circuit::zero()));
            assert!(!Circuit::is_satisfied());

            Circuit::reset_circuit();
        }
    }
}