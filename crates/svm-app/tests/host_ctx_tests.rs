use svm_app::{testing::HostCtxBuilder, types::HostCtx};

#[cfg(test)]
mod tests {
    use super::*;

    use maplit::hashmap;

    fn from_raw_parts(bytes: &Vec<u8>) -> HostCtx {
        unsafe { HostCtx::from_raw_parts(bytes.as_ptr() as _, bytes.len() as _) }.unwrap()
    }

    #[test]
    fn host_ctx_from_raw_parts_no_fields() {
        let bytes = HostCtxBuilder::new().with_version(0).build();
        let host_ctx = from_raw_parts(&bytes);

        assert_eq!(hashmap! {}, host_ctx.into_inner());
    }

    #[test]
    fn host_ctx_from_raw_parts_one_field() {
        let bytes = HostCtxBuilder::new()
            .with_version(0)
            .with_raw_field(3, &[10, 20, 30])
            .build();

        let host_ctx = from_raw_parts(&bytes);

        assert_eq!(hashmap! { 3 => vec![10, 20, 30] }, host_ctx.into_inner());
    }

    #[test]
    fn host_ctx_from_raw_parts_two_fields() {
        let bytes = HostCtxBuilder::new()
            .with_version(0)
            .with_raw_field(3, &[10, 20, 30])
            .with_raw_field(5, &[40, 50, 60, 70])
            .build();

        let host_ctx = from_raw_parts(&bytes);

        assert_eq!(
            hashmap! {
              3 => vec![10, 20, 30],
              5 => vec![40, 50, 60, 70]
            },
            host_ctx.into_inner()
        );
    }
}
