use bytes::{Buf, Bytes};
use {
    crate::{
        Context,
        Readable,
        Reader,
        Writable,
        Writer
    }
};

impl< 'a, C > Readable< 'a, C > for Bytes
    where C: Context
{
    #[inline]
    fn read_from< R: Reader< 'a, C > >( reader: &mut R ) -> Result< Self, C::Error > {
        let len = reader.read_u16()?;
        if len == 0 {
            Ok(Bytes::new())
        } else {
            let vec = reader.read_vec(len as usize)?;
            Ok(Bytes::from(vec))
        }
    }

    #[inline]
    fn minimum_bytes_needed() -> usize {
        2
    }
}


impl< C > Writable< C > for Bytes
    where C: Context
{
    #[inline]
    fn write_to< T: ?Sized + Writer< C > >( &self, writer: &mut T ) -> Result< (), C::Error > {
        writer.write_u16( self.len() as u16 )?;
        writer.write_bytes( self.chunk() )
    }

    #[inline]
    fn bytes_needed( &self ) -> Result< usize, C::Error > {
        Ok( 2 + self.len() )
    }
}
