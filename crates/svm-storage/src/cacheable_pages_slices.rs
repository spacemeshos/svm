// use super::traits::PagesStorage;
//
// #[derive(Debug, Clone, PartialEq)]
// pub struct PageSlice {
//     dirty: bool,
//     page_idx: u32,
//     offset: u32,
//     data: Vec<u8>,
// }
//
// #[derive(Debug, Clone, PartialEq)]
// pub enum CachedPageSlice {
//     /// We didn't load the page-slice yet from the underlying db
//     NotCached,
//
//     /// We've loaded page-slice from the underlying db, but no data was there
//     CachedEmpty,
//
//     /// We've loaded the page-slice from the underlying db and it had data
//     Cached(PageSlice),
// }
//
// pub struct CacheablePagesSlices<'ps, PS> {
//     /// The `ith item` will say whether the `ith page slice` is dirty
//     cached_slices: Vec<CachedPageSlice>,
//
//     cached_pages: &'ps mut PS,
// }
//
// impl<'ps, PS: PagesStorage> CacheablePagesSlices<'ps, PS> {
//     pub fn new(cached_pages: &'ps mut PS, max_page_slices: usize) -> Self {
//         Self {
//             cached_pages,
//             cached_slices: vec![CachedPageSlice::NotCached; max_page_slices],
//         }
//     }
//
//     /// * we check against `cached_slices` whether we already have the requsted page-slice
//     ///
//     /// * if we do, we return it. otherwise, we call `cached_pages.read_page()` for the page of the
//     /// given slice and cache the page slice for future use under cached_slices`
//     ///
//     /// * in case the page slice is empty we cache it as `CachedEmpty`. else, we cache it as
//     /// `Cached(PageSlice)` and mark the page-slice as non-dirty
//     pub fn read_page_slice(&mut self, _page_idx: u32, _offset: u32, _len: u32) -> Option<Vec<u8>> {
//         None
//     }
//
//     /// * we insert the new page slice into `cached_slices` as `Cached(PageSlice)` and mark it as dirty
//     pub fn write_page_slice(&mut self, _page_idx: u32, _offset: u32, _len: u32, _data: Vec<u8>) {
//         //
//     }
//
//     /// * clears the `cached_slices`
//     /// * clears the `cached_pages`, by calling `cached_pages.clear()`
//     pub fn clear(&mut self) {
//         self.cached_slices.clear();
//         self.cached_pages.clear();
//     }
//
//     /// * scans for dirty pages-slices and groups them by `page index`
//     /// * we say that a page is dirty if it has least one page-slice
//     /// * for each dirty page we read the old page data (by calling `cached_pages.read_page(..)`)
//     ///   and then we play its page-slices changes on top its old value
//     /// * for each patched cached page we do `cached_pages.write_page(..)`
//     /// * we do `cached_pages.commit()`
//     /// * we call `clear()` in order to reset the state
//     pub fn commit(&mut self) {}
// }
