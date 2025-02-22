use crate::entities::{DateFilterCondition, GridDateFilter};
use crate::services::cell::{AnyCellData, CellFilterOperation};
use crate::services::field::{DateTimestamp, DateTypeOption};
use flowy_error::FlowyResult;

impl GridDateFilter {
    pub fn is_visible<T: Into<i64>>(&self, cell_timestamp: T) -> bool {
        if self.start.is_none() {
            return false;
        }
        let cell_timestamp = cell_timestamp.into();
        let start_timestamp = *self.start.as_ref().unwrap();
        // We assume that the cell_timestamp doesn't contain hours, just day.
        match self.condition {
            DateFilterCondition::DateIs => cell_timestamp == start_timestamp,
            DateFilterCondition::DateBefore => cell_timestamp < start_timestamp,
            DateFilterCondition::DateAfter => cell_timestamp > start_timestamp,
            DateFilterCondition::DateOnOrBefore => cell_timestamp <= start_timestamp,
            DateFilterCondition::DateOnOrAfter => cell_timestamp >= start_timestamp,
            DateFilterCondition::DateWithIn => {
                if let Some(end_timestamp) = self.end.as_ref() {
                    cell_timestamp >= start_timestamp && cell_timestamp <= *end_timestamp
                } else {
                    false
                }
            }
            DateFilterCondition::DateIsEmpty => cell_timestamp == 0_i64,
        }
    }
}

impl CellFilterOperation<GridDateFilter> for DateTypeOption {
    fn apply_filter(&self, any_cell_data: AnyCellData, filter: &GridDateFilter) -> FlowyResult<bool> {
        if !any_cell_data.is_date() {
            return Ok(true);
        }
        let timestamp: DateTimestamp = any_cell_data.into();
        Ok(filter.is_visible(timestamp))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::all)]
    use crate::entities::{DateFilterCondition, GridDateFilter};

    #[test]
    fn date_filter_is_test() {
        let filter = GridDateFilter {
            condition: DateFilterCondition::DateIs,
            start: Some(123),
            end: None,
        };

        for (val, visible) in vec![(123, true), (12, false)] {
            assert_eq!(filter.is_visible(val as i64), visible);
        }
    }
    #[test]
    fn date_filter_before_test() {
        let filter = GridDateFilter {
            condition: DateFilterCondition::DateBefore,
            start: Some(123),
            end: None,
        };

        for (val, visible) in vec![(123, false), (122, true)] {
            assert_eq!(filter.is_visible(val as i64), visible);
        }
    }
    #[test]
    fn date_filter_before_or_on_test() {
        let filter = GridDateFilter {
            condition: DateFilterCondition::DateOnOrBefore,
            start: Some(123),
            end: None,
        };

        for (val, visible) in vec![(123, true), (122, true)] {
            assert_eq!(filter.is_visible(val as i64), visible);
        }
    }
    #[test]
    fn date_filter_after_test() {
        let filter = GridDateFilter {
            condition: DateFilterCondition::DateAfter,
            start: Some(123),
            end: None,
        };

        for (val, visible) in vec![(1234, true), (122, false), (0, false)] {
            assert_eq!(filter.is_visible(val as i64), visible);
        }
    }
    #[test]
    fn date_filter_within_test() {
        let filter = GridDateFilter {
            condition: DateFilterCondition::DateWithIn,
            start: Some(123),
            end: Some(130),
        };

        for (val, visible) in vec![(123, true), (130, true), (132, false)] {
            assert_eq!(filter.is_visible(val as i64), visible);
        }
    }
}
