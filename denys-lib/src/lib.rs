// SPDX-FileCopyrightText: (C) 2024 90degs2infty
// SPDX-FileContributor: 90degs2infty <90degs2infty@posteo.org>
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Library to roll off convex polygons inside one another

/// To be deleted
#[must_use]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
