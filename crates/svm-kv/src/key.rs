#[allow(unused)]
pub(crate) fn concat_ns_to_key<NS, K>(ns: NS, key: K) -> Vec<u8>
where
    NS: AsRef<[u8]>,
    K: AsRef<[u8]>,
{
    let ns = ns.as_ref();
    let key = key.as_ref();

    let cap = if ns.len() > 0 {
        ns.len() + 1 + key.len()
    } else {
        key.len()
    };

    let mut buf = Vec::with_capacity(cap);

    if ns.len() > 0 {
        buf.extend_from_slice(ns);
        buf.extend_from_slice(&[b':']);
    }

    buf.extend_from_slice(key);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_ns_to_key_empty_ns() {
        let ns = vec![];
        let key = vec![b'a', b'b', b'c'];

        let actual = concat_ns_to_key(&ns, &key);
        let expected = "abc".as_bytes();

        assert_eq!(&expected[..], &actual[..]);
    }

    #[test]
    fn concat_ns_to_key_non_empty_ns() {
        let ns = vec![b'n', b's'];
        let key = vec![b'a', b'b', b'c'];

        let actual = concat_ns_to_key(&ns, &key);
        let expected = "ns:abc".as_bytes();

        assert_eq!(&expected[..], &actual[..]);
    }
}
