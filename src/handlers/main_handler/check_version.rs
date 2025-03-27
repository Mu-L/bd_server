fn split_num(a: &String) -> Result<(u32, u32, u32), ()> {
    let mut p = if let Some(p) = a.find('.') {
        p
    } else {
        return Err(());
    };
    let (an, mut sn) = a.split_at(p);
    sn = sn.split_at(1).1;
    let an: u32 = if let Ok(r) = an.parse() {
        r
    } else {
        return Err(());
    };
    p = if let Some(p) = sn.find('.') {
        p
    } else {
        return Err(());
    };
    let (bn, mut sn) = sn.split_at(p);
    sn = sn.split_at(1).1;
    let bn: u32 = if let Ok(r) = bn.parse() {
        r
    } else {
        return Err(());
    };
    let cn: u32 = if let Ok(r) = sn.parse() {
        r
    } else {
        return Err(());
    };
    Ok((an, bn, cn))
}

/// Return true if a greater than b
pub fn version_greater(a: String, b: String) -> bool {
    let (a1, a2, a3) = if let Ok(r) = split_num(&a) {
        r
    } else {
        return false;
    };
    let (b1, b2, b3) = if let Ok(r) = split_num(&b) {
        r
    } else {
        return false;
    };
    if a1 > b1 {
        return true;
    } else if a1 < b1 {
        return false;
    }
    if a2 > b2 {
        return true;
    } else if a2 < b2 {
        return false;
    }
    if a3 > b3 {
        return true;
    }
    false
}

#[test]
fn test() {
    assert_eq!(
        version_greater("1.0.0".to_string(), "2.0.1".to_string()),
        false
    );
    assert_eq!(
        version_greater("3.0.0".to_string(), "2.0.1".to_string()),
        true
    );
    assert_eq!(
        version_greater("0.0.0".to_string(), "0.0.0".to_string()),
        false
    );
    assert_eq!(
        version_greater("1.1.1".to_string(), "1.1.0".to_string()),
        true
    );
    assert_eq!(
        version_greater("2.2.2".to_string(), "2.1.4".to_string()),
        true
    );
    assert_eq!(
        version_greater("2.3.1".to_string(), "2.9.2".to_string()),
        false
    );
}
