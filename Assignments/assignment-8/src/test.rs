#[cfg(test)]
mod test {
    use crate::custom_iterator::{GeometricSeries, Iterator};
    use crate::question_1::min_values::min_number;
    use crate::question_1::sort_element::sort_item;

    #[test]
    fn first_min_number() {
        assert_eq!(min_number(1, 2), Ok(1));
    }
    #[test]
    fn second_min_number() {
        assert_eq!(min_number(2, 2), Err("sample input".to_string()));
    }
    #[test]
    fn third_min_number() {
        assert_eq!(min_number(-1, 1), Ok(-1));
    }
    #[test]
    fn check_first_sorting() {
        let mut arr: [i32; 11] = [0, 1, 2, 3, 4, 5, 7, 9, 11, 14, 19];
        assert_eq!(
            sort_item(&mut arr).unwrap(),
            [0, 1, 2, 3, 4, 5, 7, 9, 11, 14, 19]
        );
    }

    #[test]
    fn check_second_sorting() {
        let mut arr: [f32; 9] = [0.1, 1.1, 1.2, 1.9, 3.2, 3.3, 4.1, 5.1, 6.2];
        assert_eq!(
            sort_item(&mut arr).unwrap(),
            [0.1, 1.1, 1.2, 1.9, 3.2, 3.3, 4.1, 5.1, 6.2]
        );
    }
    #[test]
    fn custom_iterator_first() {
        let mut geometric_progression = GeometricSeries {
            first_number: 1,
            current_number: 1,
            ratio: 2,
        };
        assert_eq!(
            geometric_progression.take(11).unwrap(),
            [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }

    #[test]
    fn custom_iterator_second() {
        let mut geometric_progression = GeometricSeries {
            first_number: 1,
            current_number: 1,
            ratio: 2,
        };
        assert_eq!(
            geometric_progression.take(0),
            Err("sample input".to_string())
        );
    }

    #[test]
    fn custom_iterator_third() {
        let mut geometric_progression = GeometricSeries {
            first_number: 0,
            current_number: 0,
            ratio: 0,
        };
        assert_eq!(
            geometric_progression.take(11).unwrap(),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
