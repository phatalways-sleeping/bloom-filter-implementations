use std::cell::RefCell;

pub(crate) struct Capacity {
    expected_capacity: usize,
    actual_size: RefCell<usize>,
}

impl Capacity {
    const MAXIMUM_CAPACITY_ALLOWED: usize = 10_000_000;

    pub(super) fn get_capacity(&self) -> usize {
        self.expected_capacity
    }

    pub(super) fn increase_size_by_one(&self) {
        *self.actual_size.borrow_mut() += 1;
    }
}

impl TryFrom<usize> for Capacity {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > Self::MAXIMUM_CAPACITY_ALLOWED {
            return Err("Not enough capacity for storage");
        }

        if value == 0 {
            return Err("Storage cannot have capacity of 0");
        }

        Ok(Self {
            expected_capacity: value,
            actual_size: RefCell::new(0),
        })
    }
}

#[cfg(test)]
mod test {
    use super::Capacity;

    #[test]
    fn should_return_err_when_capacity_is_invalid() {
        let expected_capacities = vec![
            0,
            Capacity::MAXIMUM_CAPACITY_ALLOWED + 1,
            Capacity::MAXIMUM_CAPACITY_ALLOWED + 100,
        ];

        for expected_capacity in expected_capacities {
            let maybe_capacity = Capacity::try_from(expected_capacity);

            assert!(maybe_capacity.is_err());
        }
    }

    #[test]
    fn should_accept_capcity_if_smaller_than_maximum_threshold() {
        let expected_capacities = vec![
            1,
            100,
            Capacity::MAXIMUM_CAPACITY_ALLOWED / 2,
            Capacity::MAXIMUM_CAPACITY_ALLOWED,
        ];

        for expected_capacity in expected_capacities {
            let maybe_capacity = Capacity::try_from(expected_capacity);

            assert!(maybe_capacity.is_ok());

            let capacity = maybe_capacity.unwrap();

            assert_eq!(capacity.expected_capacity, expected_capacity);
            assert_eq!(*capacity.actual_size.borrow(), 0);
        }
    }

    #[test]
    fn should_have_size_increased_by_one() {
        let expected_capacities = vec![
            1,
            100,
            Capacity::MAXIMUM_CAPACITY_ALLOWED / 2,
            Capacity::MAXIMUM_CAPACITY_ALLOWED,
        ];

        for expected_capacity in expected_capacities {
            let maybe_capacity = Capacity::try_from(expected_capacity);

            assert!(maybe_capacity.is_ok());

            let capacity = maybe_capacity.unwrap();

            assert_eq!(capacity.expected_capacity, expected_capacity);
            assert_eq!(*capacity.actual_size.borrow(), 0);

            capacity.increase_size_by_one();
            assert_eq!(*capacity.actual_size.borrow(), 1);

            capacity.increase_size_by_one();
            assert_eq!(*capacity.actual_size.borrow(), 2);
        }
    }
}
