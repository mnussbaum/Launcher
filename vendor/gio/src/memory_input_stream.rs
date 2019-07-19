// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>


#[cfg(test)]
mod tests {
    use glib::Bytes;
    use *;

    #[test]
    fn new() {
        let strm = MemoryInputStream::new();
        let ret = strm.skip(1, ::NONE_CANCELLABLE);
        assert!(!ret.is_err());
        assert_eq!(ret.unwrap(), 0);

        let mut buf = vec![0;10];
        let ret = strm.read(&mut buf, ::NONE_CANCELLABLE).unwrap();
        assert_eq!(ret, 0);
    }

    #[test]
    fn new_from_bytes() {
        let b = Bytes::from_owned(vec![1, 2, 3]);
        let strm = MemoryInputStream::new_from_bytes(&b);
        let mut buf = vec![0;10];
        let ret = strm.read(&mut buf, ::NONE_CANCELLABLE).unwrap();
        assert_eq!(ret, 3);
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);

        let ret = strm.skip(10, ::NONE_CANCELLABLE).unwrap();
        assert_eq!(ret, 0);
    }

    #[test]
    fn add_bytes() {
        let strm = MemoryInputStream::new();
        let b = Bytes::from_owned(vec![1, 2, 3]);
        strm.add_bytes(&b);
        let mut buf = vec![0;10];
        let ret = strm.read(&mut buf, ::NONE_CANCELLABLE).unwrap();
        assert_eq!(ret, 3);
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);

        let ret = strm.skip(10, ::NONE_CANCELLABLE).unwrap();
        assert_eq!(ret, 0);
    }

    #[test]
    #[cfg(feature = "futures")]
    fn read_async_future() {
        use futures_util::FutureExt;

        let c = glib::MainContext::new();

        let buf = vec![0; 10];
        let b = glib::Bytes::from_owned(vec![1, 2, 3]);
        let strm = MemoryInputStream::new_from_bytes(&b);

        let res = c.block_on(
            strm.read_async_future(buf, glib::PRIORITY_DEFAULT)
                .map_err(|(_obj, (_buf, err))| err)
                .and_then(move |(_obj, (mut buf, len))| {
                    buf.truncate(len);
                    Ok(buf)
                })
        ).unwrap();

        assert_eq!(res, vec![1, 2, 3]);
    }
}
