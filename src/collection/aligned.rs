
/// Is the collection alligned to a given alignment
unsafe trait ByteAlignedCollection<const Alignment: usize> {

}

impl ByteAlignedCollection for AlignedVec {

}