use serde_json::{Map, Value};

/// Filters a JSON value by keeping only the specified top-level keys.
/// If the value is not an object, it's returned as-is.
/// If fields is empty, it returns the original object.
pub fn filter_fields(data: &Value, fields: &[String]) -> Value {
    if fields.is_empty() {
        return data.clone();
    }

    match data {
        Value::Object(obj) => {
            let mut filtered = Map::new();
            for field in fields {
                if let Some(val) = obj.get(field) {
                    filtered.insert(field.clone(), val.clone());
                }
            }
            Value::Object(filtered)
        }
        _ => data.clone(),
    }
}

/// Paginates a vector of items.
/// Page is 1-indexed.
pub fn paginate_vec<T>(items: &[T], page: usize, per_page: usize) -> (Vec<&T>, usize) {
    let total = items.len();
    if per_page == 0 || page == 0 {
        return (vec![], total);
    }

    let start = (page - 1) * per_page;
    if start >= total {
        return (vec![], total);
    }

    let end = (start + per_page).min(total);
    let paged = items[start..end].iter().collect();

    (paged, total)
}

/// Paginates any collection that can be iterated as pairs (like HashMap or BTreeMap).
pub fn paginate_map<'a, K, V, I>(items: I, page: usize, per_page: usize) -> (Vec<(&'a K, &'a V)>, usize)
where
    I: IntoIterator<Item = (&'a K, &'a V)>,
{
    let all_items: Vec<(&'a K, &'a V)> = items.into_iter().collect();
    let total = all_items.len();

    if per_page == 0 || page == 0 {
        return (vec![], total);
    }

    let start = (page - 1) * per_page;
    if start >= total {
        return (vec![], total);
    }

    let end = (start + per_page).min(total);
    let paged = all_items[start..end].iter().cloned().collect();

    (paged, total)
}
