use super::*;
use rayon::iter::ParallelIterator;
use std::fmt::Debug;
use std::ops::Add;
use std::str::FromStr;

pub(crate) fn parse_types<TypeT: FromStr + ToFromUsize + Sync + Send + Debug + Add<Output=TypeT>>(
    types_iterator: Option<impl ParallelIterator<Item = Result<String>>>,
    types_number: Option<TypeT>,
    numeric_ids: bool,
    minimum_id: Option<TypeT>,
    has_node_types: bool,
) -> Result<Option<Vocabulary<TypeT>>> {
    // when the graph has no node_types, the resulting vocabulary is None
    if !has_node_types {
        return Ok(None);
    }

    if !numeric_ids && minimum_id.is_some() {
        return Err(
            "Giving the minimum id is not meaningfull when numeric_ids is false.".to_string(),
        );
    }

    match (types_iterator, types_number, numeric_ids, minimum_id) {
        (Some(nti), _, false, None) => Ok(Some(Vocabulary::from_reverse_map(
            nti.collect::<Result<Vec<String>>>()?,
        )?)),
        (Some(nti), None, true, _) => {
            let (min, max) = nti
                .map(|type_name| match type_name {
                    Ok(tn) => match tn.parse::<TypeT>() {
                        Ok(type_id) => Ok(type_id),
                        Err(_) => Err(format!(
                            concat!(
                                "While parsing the provided node type list, ",
                                "the node type ID {:?} was found and it is not ",
                                "possible to convert it to an integer as was requested.",
                            ),
                            tn
                        )),
                    },
                    Err(e) => Err(e),
                })
                .map(|maybe_type_id| maybe_type_id.map(|type_id| (type_id, type_id)))
                .reduce(
                    || Ok((TypeT::get_max(), TypeT::from_usize(0))),
                    |v1, v2| match (v1, v2) {
                        (Ok((min1, max1)), Ok((min2, max2))) => {
                            Ok((min1.min(min2), max1.max(max2)))
                        }
                        (Ok((min1, max1)), Err(e2)) => Ok((min1, max1)),
                        (Err(e1), Ok((min2, max2))) => Ok((min2, max2)),
                        (Err(e1), Err(e2)) => Err(e1),
                    },
                )?;
            let minimum_node_ids = minimum_id.unwrap_or(min);

            if min < minimum_node_ids {
                return Err(format!(
                    concat!(
                        "The given minimum id {:?} is higher ",
                        "than the minimum id found in the iterator {:?}."
                    ),
                    minimum_node_ids, min
                ));
            }

            Ok(Some(Vocabulary::from_range(minimum_node_ids..max)))
        }
        (None, Some(ntn), true, None) => {
            Ok(Some(Vocabulary::from_range(TypeT::from_usize(0)..ntn)))
        }
        (None, Some(ntn), true, Some(min_val)) => {
            Ok(Some(Vocabulary::from_range(min_val..(min_val + ntn))))
        }
        (None, Some(ntn), true, _) => {
            let min = minimum_id.unwrap_or(TypeT::from_usize(0));
            Ok(Some(Vocabulary::from_range(min..min)))
        }
        (None, Some(ntn), false, None) => Ok(Some(Vocabulary::with_capacity(TypeT::to_usize(ntn)))),
        (None, None, false, None) => Ok(Some(Vocabulary::new())),
        _ => unreachable!("All other cases must be explictily handled."),
    }
}
