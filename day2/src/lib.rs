pub fn check_for_safety(levels: &[i32]) -> bool {
    let mut is_increasing = None::<bool>;
    let mut previous_level = None::<i32>;

    for level in levels.iter().copied() {
        if let Some(previous) = previous_level {
            match is_increasing.get_or_insert(level > previous) {
                true => {
                    if level <= previous || level - previous > 3 {
                        return false;
                    }
                }
                false => {
                    if level >= previous || previous - level > 3 {
                        return false;
                    }
                }
            }
        }
        previous_level = Some(level);
    }

    true
}
