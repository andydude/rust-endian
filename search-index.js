var searchIndex = {};
searchIndex['endian'] = {"items":[[0,"","endian",""],[8,"ReadEndian","","Read extension for endianness."],[10,"read_be","","",0],[10,"read_le","","",0],[8,"WriteEndian","","Write extension for endianness."],[10,"write_be","","",1],[10,"write_le","","",1],[11,"read_be","collections::vec","",2],[11,"read_le","","",2],[11,"write_be","","",2],[11,"write_le","","",2],[11,"read_be","","",2],[11,"read_le","","",2],[11,"write_be","","",2],[11,"write_le","","",2],[11,"read_be","","",2],[11,"read_le","","",2],[11,"write_be","","",2],[11,"write_le","","",2]],"paths":[[8,"ReadEndian"],[8,"WriteEndian"],[3,"Vec"]]};
searchIndex['bswap'] = {"items":[[0,"","bswap",""],[0,"u8","","Swap bytes for `u8` slices on all targets."],[5,"align_of_ptr","bswap::u8","TODO"],[5,"reverse_slice_inplace","","TODO"],[5,"reverse_slice","","TODO"],[5,"reverse_memory_inplace","","TODO"],[5,"reverse_memory","","TODO"],[5,"reverse_memory_array_inplace","",""],[5,"reverse_memory_array","",""],[17,"BYTES","",""],[0,"u16","bswap","Swap bytes for `u16` objects on all targets."],[5,"swap_memory_inplace","bswap::u16","TODO"],[5,"swap_memory","","TODO"],[17,"BYTES","",""],[0,"u24","bswap","Swap bytes for `[u8; 3]` objects on all targets."],[5,"swap_memory_inplace","bswap::u24",""],[5,"swap_memory","",""],[17,"BYTES","",""],[0,"u40","bswap","Swap bytes for `[u8; 5]` objects on all targets."],[5,"swap_memory_inplace","bswap::u40",""],[5,"swap_memory","",""],[17,"BYTES","",""],[0,"u48","bswap","Swap bytes for `[u8; 6]` objects on all targets."],[5,"swap_memory_inplace","bswap::u48",""],[5,"swap_memory","",""],[17,"BYTES","",""],[0,"u56","bswap","Swap bytes for `[u8; 7]` objects on all targets."],[5,"swap_memory_inplace","bswap::u56",""],[5,"swap_memory","",""],[17,"BYTES","",""],[0,"u32","bswap","Swap bytes for `u32` objects on all targets."],[5,"swap_memory_inplace","bswap::u32","Swaps `len*4` bytes for `u32` objects inplace in `buf`."],[5,"swap_memory","","Swaps `len*4` bytes for `u32` objects from `src` to `dst`. The source and destination may not overlap."],[17,"BYTES","",""],[0,"u64","bswap","Swap bytes for `u64` objects on all targets."],[5,"swap_memory_inplace","bswap::u64","Swaps `len*8` bytes for `u64` objects inplace in `buf`."],[5,"swap_memory","","Swaps `len*8` bytes for `u64` objects from `src` to `dst`. The source and destination may not overlap."],[17,"BYTES","",""],[0,"beusize","bswap",""],[5,"decode","bswap::beusize",""],[5,"encode","",""],[0,"leusize","bswap",""],[5,"decode","bswap::leusize",""],[5,"encode","",""],[0,"beu16","bswap","Swap bytes for `u16` objects only on little-endian targets, does nothing on big-endian targets."],[5,"decode","bswap::beu16","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"beu24","bswap","Swap bytes for `[u8; 3]` objects only on little-endian targets, does nothing on big-endian targets."],[5,"decode","bswap::beu24","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"beu32","bswap","Swap bytes for `u32` objects only on little-endian targets, does nothing on big-endian targets."],[5,"decode","bswap::beu32","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"beu40","bswap","Swap bytes for `[u8; 5]` objects only on little-endian targets, does nothing on big-endian targets."],[5,"decode","bswap::beu40","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"beu48","bswap","Swap bytes for `[u8; 6]` objects only on little-endian targets, does nothing on big-endian targets."],[5,"decode","bswap::beu48","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"beu56","bswap","Swap bytes for `[u8; 7]` objects only on little-endian targets, does nothing on big-endian targets."],[5,"decode","bswap::beu56","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"beu64","bswap","Swap bytes for `u64` objects only on little-endian targets, does nothing on big-endian targets."],[5,"decode","bswap::beu64","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"leu16","bswap","Swap bytes for `u16` objects only on big-endian targets, does nothing on little-endian targets."],[5,"decode","bswap::leu16","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"leu24","bswap","Swap bytes for `[u8; 3]` objects only on big-endian targets, does nothing on little-endian targets."],[5,"decode","bswap::leu24","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"leu32","bswap","Swap bytes for `u32` objects only on big-endian targets, does nothing on little-endian targets."],[5,"decode","bswap::leu32","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"leu40","bswap","Swap bytes for `[u8; 5]` objects only on big-endian targets, does nothing on little-endian targets."],[5,"decode","bswap::leu40","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"leu48","bswap","Swap bytes for `[u8; 6]` objects only on big-endian targets, does nothing on little-endian targets."],[5,"decode","bswap::leu48","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"leu56","bswap","Swap bytes for `[u8; 7]` objects only on big-endian targets, does nothing on little-endian targets."],[5,"decode","bswap::leu56","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."],[0,"leu64","bswap","Swap bytes for `u64` objects only on big-endian targets, does nothing on little-endian targets."],[5,"decode","bswap::leu64","Decodes $E-endian bytes to a native-endian $T object."],[5,"decode_slice","","Decodes $E-endian bytes to a slice of native-endian $T objects."],[5,"encode","","Encodes a native-endian $T object to $E-endian bytes."],[5,"encode_slice","","Encodes a slice of native-endian $T objects to $E-endian bytes."]],"paths":[]};

initSearch(searchIndex);