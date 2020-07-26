pub fn cipher_128(state: (u32, &str)) -> Vec<u8> {
    match state {
        
        ( 0, "input") =>  vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff],
        ( 0, "k_sch") =>  vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f],
        ( 1, "start") =>  vec![0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0],
        ( 1, "s_box") =>  vec![0x63, 0xca, 0xb7, 0x04, 0x09, 0x53, 0xd0, 0x51, 0xcd, 0x60, 0xe0, 0xe7, 0xba, 0x70, 0xe1, 0x8c],
        ( 1, "s_row") =>  vec![0x63, 0x53, 0xe0, 0x8c, 0x09, 0x60, 0xe1, 0x04, 0xcd, 0x70, 0xb7, 0x51, 0xba, 0xca, 0xd0, 0xe7],
        ( 1, "m_col") =>  vec![0x5f, 0x72, 0x64, 0x15, 0x57, 0xf5, 0xbc, 0x92, 0xf7, 0xbe, 0x3b, 0x29, 0x1d, 0xb9, 0xf9, 0x1a],
        ( 1, "k_sch") =>  vec![0xd6, 0xaa, 0x74, 0xfd, 0xd2, 0xaf, 0x72, 0xfa, 0xda, 0xa6, 0x78, 0xf1, 0xd6, 0xab, 0x76, 0xfe],
        ( 2, "start") =>  vec![0x89, 0xd8, 0x10, 0xe8, 0x85, 0x5a, 0xce, 0x68, 0x2d, 0x18, 0x43, 0xd8, 0xcb, 0x12, 0x8f, 0xe4],
        ( 2, "s_box") =>  vec![0xa7, 0x61, 0xca, 0x9b, 0x97, 0xbe, 0x8b, 0x45, 0xd8, 0xad, 0x1a, 0x61, 0x1f, 0xc9, 0x73, 0x69],
        ( 2, "s_row") =>  vec![0xa7, 0xbe, 0x1a, 0x69, 0x97, 0xad, 0x73, 0x9b, 0xd8, 0xc9, 0xca, 0x45, 0x1f, 0x61, 0x8b, 0x61],
        ( 2, "m_col") =>  vec![0xff, 0x87, 0x96, 0x84, 0x31, 0xd8, 0x6a, 0x51, 0x64, 0x51, 0x51, 0xfa, 0x77, 0x3a, 0xd0, 0x09],
        ( 2, "k_sch") =>  vec![0xb6, 0x92, 0xcf, 0x0b, 0x64, 0x3d, 0xbd, 0xf1, 0xbe, 0x9b, 0xc5, 0x00, 0x68, 0x30, 0xb3, 0xfe],
        ( 3, "start") =>  vec![0x49, 0x15, 0x59, 0x8f, 0x55, 0xe5, 0xd7, 0xa0, 0xda, 0xca, 0x94, 0xfa, 0x1f, 0x0a, 0x63, 0xf7],
        ( 3, "s_box") =>  vec![0x3b, 0x59, 0xcb, 0x73, 0xfc, 0xd9, 0x0e, 0xe0, 0x57, 0x74, 0x22, 0x2d, 0xc0, 0x67, 0xfb, 0x68],
        ( 3, "s_row") =>  vec![0x3b, 0xd9, 0x22, 0x68, 0xfc, 0x74, 0xfb, 0x73, 0x57, 0x67, 0xcb, 0xe0, 0xc0, 0x59, 0x0e, 0x2d],
        ( 3, "m_col") =>  vec![0x4c, 0x9c, 0x1e, 0x66, 0xf7, 0x71, 0xf0, 0x76, 0x2c, 0x3f, 0x86, 0x8e, 0x53, 0x4d, 0xf2, 0x56],
        ( 3, "k_sch") =>  vec![0xb6, 0xff, 0x74, 0x4e, 0xd2, 0xc2, 0xc9, 0xbf, 0x6c, 0x59, 0x0c, 0xbf, 0x04, 0x69, 0xbf, 0x41],
        ( 4, "start") =>  vec![0xfa, 0x63, 0x6a, 0x28, 0x25, 0xb3, 0x39, 0xc9, 0x40, 0x66, 0x8a, 0x31, 0x57, 0x24, 0x4d, 0x17],
        ( 4, "s_box") =>  vec![0x2d, 0xfb, 0x02, 0x34, 0x3f, 0x6d, 0x12, 0xdd, 0x09, 0x33, 0x7e, 0xc7, 0x5b, 0x36, 0xe3, 0xf0],
        ( 4, "s_row") =>  vec![0x2d, 0x6d, 0x7e, 0xf0, 0x3f, 0x33, 0xe3, 0x34, 0x09, 0x36, 0x02, 0xdd, 0x5b, 0xfb, 0x12, 0xc7],
        ( 4, "m_col") =>  vec![0x63, 0x85, 0xb7, 0x9f, 0xfc, 0x53, 0x8d, 0xf9, 0x97, 0xbe, 0x47, 0x8e, 0x75, 0x47, 0xd6, 0x91],
        ( 4, "k_sch") =>  vec![0x47, 0xf7, 0xf7, 0xbc, 0x95, 0x35, 0x3e, 0x03, 0xf9, 0x6c, 0x32, 0xbc, 0xfd, 0x05, 0x8d, 0xfd],
        ( 5, "start") =>  vec![0x24, 0x72, 0x40, 0x23, 0x69, 0x66, 0xb3, 0xfa, 0x6e, 0xd2, 0x75, 0x32, 0x88, 0x42, 0x5b, 0x6c],
        ( 5, "s_box") =>  vec![0x36, 0x40, 0x09, 0x26, 0xf9, 0x33, 0x6d, 0x2d, 0x9f, 0xb5, 0x9d, 0x23, 0xc4, 0x2c, 0x39, 0x50],
        ( 5, "s_row") =>  vec![0x36, 0x33, 0x9d, 0x50, 0xf9, 0xb5, 0x39, 0x26, 0x9f, 0x2c, 0x09, 0x2d, 0xc4, 0x40, 0x6d, 0x23],
        ( 5, "m_col") =>  vec![0xf4, 0xbc, 0xd4, 0x54, 0x32, 0xe5, 0x54, 0xd0, 0x75, 0xf1, 0xd6, 0xc5, 0x1d, 0xd0, 0x3b, 0x3c],
        ( 5, "k_sch") =>  vec![0x3c, 0xaa, 0xa3, 0xe8, 0xa9, 0x9f, 0x9d, 0xeb, 0x50, 0xf3, 0xaf, 0x57, 0xad, 0xf6, 0x22, 0xaa],
        ( 6, "start") =>  vec![0xc8, 0x16, 0x77, 0xbc, 0x9b, 0x7a, 0xc9, 0x3b, 0x25, 0x02, 0x79, 0x92, 0xb0, 0x26, 0x19, 0x96],
        ( 6, "s_box") =>  vec![0xe8, 0x47, 0xf5, 0x65, 0x14, 0xda, 0xdd, 0xe2, 0x3f, 0x77, 0xb6, 0x4f, 0xe7, 0xf7, 0xd4, 0x90],
        ( 6, "s_row") =>  vec![0xe8, 0xda, 0xb6, 0x90, 0x14, 0x77, 0xd4, 0x65, 0x3f, 0xf7, 0xf5, 0xe2, 0xe7, 0x47, 0xdd, 0x4f],
        ( 6, "m_col") =>  vec![0x98, 0x16, 0xee, 0x74, 0x00, 0xf8, 0x7f, 0x55, 0x6b, 0x2c, 0x04, 0x9c, 0x8e, 0x5a, 0xd0, 0x36],
        ( 6, "k_sch") =>  vec![0x5e, 0x39, 0x0f, 0x7d, 0xf7, 0xa6, 0x92, 0x96, 0xa7, 0x55, 0x3d, 0xc1, 0x0a, 0xa3, 0x1f, 0x6b],
        ( 7, "start") =>  vec![0xc6, 0x2f, 0xe1, 0x09, 0xf7, 0x5e, 0xed, 0xc3, 0xcc, 0x79, 0x39, 0x5d, 0x84, 0xf9, 0xcf, 0x5d],
        ( 7, "s_box") =>  vec![0xb4, 0x15, 0xf8, 0x01, 0x68, 0x58, 0x55, 0x2e, 0x4b, 0xb6, 0x12, 0x4c, 0x5f, 0x99, 0x8a, 0x4c],
        ( 7, "s_row") =>  vec![0xb4, 0x58, 0x12, 0x4c, 0x68, 0xb6, 0x8a, 0x01, 0x4b, 0x99, 0xf8, 0x2e, 0x5f, 0x15, 0x55, 0x4c],
        ( 7, "m_col") =>  vec![0xc5, 0x7e, 0x1c, 0x15, 0x9a, 0x9b, 0xd2, 0x86, 0xf0, 0x5f, 0x4b, 0xe0, 0x98, 0xc6, 0x34, 0x39],
        ( 7, "k_sch") =>  vec![0x14, 0xf9, 0x70, 0x1a, 0xe3, 0x5f, 0xe2, 0x8c, 0x44, 0x0a, 0xdf, 0x4d, 0x4e, 0xa9, 0xc0, 0x26],
        ( 8, "start") =>  vec![0xd1, 0x87, 0x6c, 0x0f, 0x79, 0xc4, 0x30, 0x0a, 0xb4, 0x55, 0x94, 0xad, 0xd6, 0x6f, 0xf4, 0x1f],
        ( 8, "s_box") =>  vec![0x3e, 0x17, 0x50, 0x76, 0xb6, 0x1c, 0x04, 0x67, 0x8d, 0xfc, 0x22, 0x95, 0xf6, 0xa8, 0xbf, 0xc0],
        ( 8, "s_row") =>  vec![0x3e, 0x1c, 0x22, 0xc0, 0xb6, 0xfc, 0xbf, 0x76, 0x8d, 0xa8, 0x50, 0x67, 0xf6, 0x17, 0x04, 0x95],
        ( 8, "m_col") =>  vec![0xba, 0xa0, 0x3d, 0xe7, 0xa1, 0xf9, 0xb5, 0x6e, 0xd5, 0x51, 0x2c, 0xba, 0x5f, 0x41, 0x4d, 0x23],
        ( 8, "k_sch") =>  vec![0x47, 0x43, 0x87, 0x35, 0xa4, 0x1c, 0x65, 0xb9, 0xe0, 0x16, 0xba, 0xf4, 0xae, 0xbf, 0x7a, 0xd2],
        ( 9, "start") =>  vec![0xfd, 0xe3, 0xba, 0xd2, 0x05, 0xe5, 0xd0, 0xd7, 0x35, 0x47, 0x96, 0x4e, 0xf1, 0xfe, 0x37, 0xf1],
        ( 9, "s_box") =>  vec![0x54, 0x11, 0xf4, 0xb5, 0x6b, 0xd9, 0x70, 0x0e, 0x96, 0xa0, 0x90, 0x2f, 0xa1, 0xbb, 0x9a, 0xa1],
        ( 9, "s_row") =>  vec![0x54, 0xd9, 0x90, 0xa1, 0x6b, 0xa0, 0x9a, 0xb5, 0x96, 0xbb, 0xf4, 0x0e, 0xa1, 0x11, 0x70, 0x2f],
        ( 9, "m_col") =>  vec![0xe9, 0xf7, 0x4e, 0xec, 0x02, 0x30, 0x20, 0xf6, 0x1b, 0xf2, 0xcc, 0xf2, 0x35, 0x3c, 0x21, 0xc7],
        ( 9, "k_sch") =>  vec![0x54, 0x99, 0x32, 0xd1, 0xf0, 0x85, 0x57, 0x68, 0x10, 0x93, 0xed, 0x9c, 0xbe, 0x2c, 0x97, 0x4e],
        (10, "start") =>  vec![0xbd, 0x6e, 0x7c, 0x3d, 0xf2, 0xb5, 0x77, 0x9e, 0x0b, 0x61, 0x21, 0x6e, 0x8b, 0x10, 0xb6, 0x89],
        (10, "s_box") =>  vec![0x7a, 0x9f, 0x10, 0x27, 0x89, 0xd5, 0xf5, 0x0b, 0x2b, 0xef, 0xfd, 0x9f, 0x3d, 0xca, 0x4e, 0xa7],
        (10, "s_row") =>  vec![0x7a, 0xd5, 0xfd, 0xa7, 0x89, 0xef, 0x4e, 0x27, 0x2b, 0xca, 0x10, 0x0b, 0x3d, 0x9f, 0xf5, 0x9f],
        (10, "k_sch") =>  vec![0x13, 0x11, 0x1d, 0x7f, 0xe3, 0x94, 0x4a, 0x17, 0xf3, 0x07, 0xa7, 0x8b, 0x4d, 0x2b, 0x30, 0xc5],
        (10, "output") => vec![0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4, 0xc5, 0x5a],
        _           => panic!("error in lookup cipher 128 test values"),
    }
}

pub fn inv_cipher_128(state: (u32, &str)) -> Vec<u8> {
    match state {
        ( 0,"iinput")   =>  vec![0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4, 0xc5, 0x5a],
        ( 0,"ik_sch")   =>  vec![0x13, 0x11, 0x1d, 0x7f, 0xe3, 0x94, 0x4a, 0x17, 0xf3, 0x07, 0xa7, 0x8b, 0x4d, 0x2b, 0x30, 0xc5],
        ( 1,"istart")   =>  vec![0x7a, 0xd5, 0xfd, 0xa7, 0x89, 0xef, 0x4e, 0x27, 0x2b, 0xca, 0x10, 0x0b, 0x3d, 0x9f, 0xf5, 0x9f],
        ( 1,"is_row")   =>  vec![0x7a, 0x9f, 0x10, 0x27, 0x89, 0xd5, 0xf5, 0x0b, 0x2b, 0xef, 0xfd, 0x9f, 0x3d, 0xca, 0x4e, 0xa7],
        ( 1,"is_box")   =>  vec![0xbd, 0x6e, 0x7c, 0x3d, 0xf2, 0xb5, 0x77, 0x9e, 0x0b, 0x61, 0x21, 0x6e, 0x8b, 0x10, 0xb6, 0x89],
        ( 1,"ik_sch")   =>  vec![0x54, 0x99, 0x32, 0xd1, 0xf0, 0x85, 0x57, 0x68, 0x10, 0x93, 0xed, 0x9c, 0xbe, 0x2c, 0x97, 0x4e],
        ( 1,"ik_add")   =>  vec![0xe9, 0xf7, 0x4e, 0xec, 0x02, 0x30, 0x20, 0xf6, 0x1b, 0xf2, 0xcc, 0xf2, 0x35, 0x3c, 0x21, 0xc7],
        ( 2,"istart")   =>  vec![0x54, 0xd9, 0x90, 0xa1, 0x6b, 0xa0, 0x9a, 0xb5, 0x96, 0xbb, 0xf4, 0x0e, 0xa1, 0x11, 0x70, 0x2f],
        ( 2,"is_row")   =>  vec![0x54, 0x11, 0xf4, 0xb5, 0x6b, 0xd9, 0x70, 0x0e, 0x96, 0xa0, 0x90, 0x2f, 0xa1, 0xbb, 0x9a, 0xa1],
        ( 2,"is_box")   =>  vec![0xfd, 0xe3, 0xba, 0xd2, 0x05, 0xe5, 0xd0, 0xd7, 0x35, 0x47, 0x96, 0x4e, 0xf1, 0xfe, 0x37, 0xf1],
        ( 2,"ik_sch")   =>  vec![0x47, 0x43, 0x87, 0x35, 0xa4, 0x1c, 0x65, 0xb9, 0xe0, 0x16, 0xba, 0xf4, 0xae, 0xbf, 0x7a, 0xd2],
        ( 2,"ik_add")   =>  vec![0xba, 0xa0, 0x3d, 0xe7, 0xa1, 0xf9, 0xb5, 0x6e, 0xd5, 0x51, 0x2c, 0xba, 0x5f, 0x41, 0x4d, 0x23],
        ( 3,"istart")   =>  vec![0x3e, 0x1c, 0x22, 0xc0, 0xb6, 0xfc, 0xbf, 0x76, 0x8d, 0xa8, 0x50, 0x67, 0xf6, 0x17, 0x04, 0x95],
        ( 3,"is_row")   =>  vec![0x3e, 0x17, 0x50, 0x76, 0xb6, 0x1c, 0x04, 0x67, 0x8d, 0xfc, 0x22, 0x95, 0xf6, 0xa8, 0xbf, 0xc0],
        ( 3,"is_box")   =>  vec![0xd1, 0x87, 0x6c, 0x0f, 0x79, 0xc4, 0x30, 0x0a, 0xb4, 0x55, 0x94, 0xad, 0xd6, 0x6f, 0xf4, 0x1f],
        ( 3,"ik_sch")   =>  vec![0x14, 0xf9, 0x70, 0x1a, 0xe3, 0x5f, 0xe2, 0x8c, 0x44, 0x0a, 0xdf, 0x4d, 0x4e, 0xa9, 0xc0, 0x26],
        ( 3,"ik_add")   =>  vec![0xc5, 0x7e, 0x1c, 0x15, 0x9a, 0x9b, 0xd2, 0x86, 0xf0, 0x5f, 0x4b, 0xe0, 0x98, 0xc6, 0x34, 0x39],
        ( 4,"istart")   =>  vec![0xb4, 0x58, 0x12, 0x4c, 0x68, 0xb6, 0x8a, 0x01, 0x4b, 0x99, 0xf8, 0x2e, 0x5f, 0x15, 0x55, 0x4c],
        ( 4,"is_row")   =>  vec![0xb4, 0x15, 0xf8, 0x01, 0x68, 0x58, 0x55, 0x2e, 0x4b, 0xb6, 0x12, 0x4c, 0x5f, 0x99, 0x8a, 0x4c],
        ( 4,"is_box")   =>  vec![0xc6, 0x2f, 0xe1, 0x09, 0xf7, 0x5e, 0xed, 0xc3, 0xcc, 0x79, 0x39, 0x5d, 0x84, 0xf9, 0xcf, 0x5d],
        ( 4,"ik_sch")   =>  vec![0x5e, 0x39, 0x0f, 0x7d, 0xf7, 0xa6, 0x92, 0x96, 0xa7, 0x55, 0x3d, 0xc1, 0x0a, 0xa3, 0x1f, 0x6b],
        ( 4,"ik_add")   =>  vec![0x98, 0x16, 0xee, 0x74, 0x00, 0xf8, 0x7f, 0x55, 0x6b, 0x2c, 0x04, 0x9c, 0x8e, 0x5a, 0xd0, 0x36],
        ( 5,"istart")   =>  vec![0xe8, 0xda, 0xb6, 0x90, 0x14, 0x77, 0xd4, 0x65, 0x3f, 0xf7, 0xf5, 0xe2, 0xe7, 0x47, 0xdd, 0x4f],
        ( 5,"is_row")   =>  vec![0xe8, 0x47, 0xf5, 0x65, 0x14, 0xda, 0xdd, 0xe2, 0x3f, 0x77, 0xb6, 0x4f, 0xe7, 0xf7, 0xd4, 0x90],
        ( 5,"is_box")   =>  vec![0xc8, 0x16, 0x77, 0xbc, 0x9b, 0x7a, 0xc9, 0x3b, 0x25, 0x02, 0x79, 0x92, 0xb0, 0x26, 0x19, 0x96],
        ( 5,"ik_sch")   =>  vec![0x3c, 0xaa, 0xa3, 0xe8, 0xa9, 0x9f, 0x9d, 0xeb, 0x50, 0xf3, 0xaf, 0x57, 0xad, 0xf6, 0x22, 0xaa],
        ( 5,"ik_add")   =>  vec![0xf4, 0xbc, 0xd4, 0x54, 0x32, 0xe5, 0x54, 0xd0, 0x75, 0xf1, 0xd6, 0xc5, 0x1d, 0xd0, 0x3b, 0x3c],
        ( 6,"istart")   =>  vec![0x36, 0x33, 0x9d, 0x50, 0xf9, 0xb5, 0x39, 0x26, 0x9f, 0x2c, 0x09, 0x2d, 0xc4, 0x40, 0x6d, 0x23],
        ( 6,"is_row")   =>  vec![0x36, 0x40, 0x09, 0x26, 0xf9, 0x33, 0x6d, 0x2d, 0x9f, 0xb5, 0x9d, 0x23, 0xc4, 0x2c, 0x39, 0x50],
        ( 6,"is_box")   =>  vec![0x24, 0x72, 0x40, 0x23, 0x69, 0x66, 0xb3, 0xfa, 0x6e, 0xd2, 0x75, 0x32, 0x88, 0x42, 0x5b, 0x6c],
        ( 6,"ik_sch")   =>  vec![0x47, 0xf7, 0xf7, 0xbc, 0x95, 0x35, 0x3e, 0x03, 0xf9, 0x6c, 0x32, 0xbc, 0xfd, 0x05, 0x8d, 0xfd],
        ( 6,"ik_add")   =>  vec![0x63, 0x85, 0xb7, 0x9f, 0xfc, 0x53, 0x8d, 0xf9, 0x97, 0xbe, 0x47, 0x8e, 0x75, 0x47, 0xd6, 0x91],
        ( 7,"istart")   =>  vec![0x2d, 0x6d, 0x7e, 0xf0, 0x3f, 0x33, 0xe3, 0x34, 0x09, 0x36, 0x02, 0xdd, 0x5b, 0xfb, 0x12, 0xc7],
        ( 7,"is_row")   =>  vec![0x2d, 0xfb, 0x02, 0x34, 0x3f, 0x6d, 0x12, 0xdd, 0x09, 0x33, 0x7e, 0xc7, 0x5b, 0x36, 0xe3, 0xf0],
        ( 7,"is_box")   =>  vec![0xfa, 0x63, 0x6a, 0x28, 0x25, 0xb3, 0x39, 0xc9, 0x40, 0x66, 0x8a, 0x31, 0x57, 0x24, 0x4d, 0x17],
        ( 7,"ik_sch")   =>  vec![0xb6, 0xff, 0x74, 0x4e, 0xd2, 0xc2, 0xc9, 0xbf, 0x6c, 0x59, 0x0c, 0xbf, 0x04, 0x69, 0xbf, 0x41],
        ( 7,"ik_add")   =>  vec![0x4c, 0x9c, 0x1e, 0x66, 0xf7, 0x71, 0xf0, 0x76, 0x2c, 0x3f, 0x86, 0x8e, 0x53, 0x4d, 0xf2, 0x56],
        ( 8,"istart")   =>  vec![0x3b, 0xd9, 0x22, 0x68, 0xfc, 0x74, 0xfb, 0x73, 0x57, 0x67, 0xcb, 0xe0, 0xc0, 0x59, 0x0e, 0x2d],
        ( 8,"is_row")   =>  vec![0x3b, 0x59, 0xcb, 0x73, 0xfc, 0xd9, 0x0e, 0xe0, 0x57, 0x74, 0x22, 0x2d, 0xc0, 0x67, 0xfb, 0x68],
        ( 8,"is_box")   =>  vec![0x49, 0x15, 0x59, 0x8f, 0x55, 0xe5, 0xd7, 0xa0, 0xda, 0xca, 0x94, 0xfa, 0x1f, 0x0a, 0x63, 0xf7],
        ( 8,"ik_sch")   =>  vec![0xb6, 0x92, 0xcf, 0x0b, 0x64, 0x3d, 0xbd, 0xf1, 0xbe, 0x9b, 0xc5, 0x00, 0x68, 0x30, 0xb3, 0xfe],
        ( 8,"ik_add")   =>  vec![0xff, 0x87, 0x96, 0x84, 0x31, 0xd8, 0x6a, 0x51, 0x64, 0x51, 0x51, 0xfa, 0x77, 0x3a, 0xd0, 0x09],
        ( 9,"istart")   =>  vec![0xa7, 0xbe, 0x1a, 0x69, 0x97, 0xad, 0x73, 0x9b, 0xd8, 0xc9, 0xca, 0x45, 0x1f, 0x61, 0x8b, 0x61],
        ( 9,"is_row")   =>  vec![0xa7, 0x61, 0xca, 0x9b, 0x97, 0xbe, 0x8b, 0x45, 0xd8, 0xad, 0x1a, 0x61, 0x1f, 0xc9, 0x73, 0x69],
        ( 9,"is_box")   =>  vec![0x89, 0xd8, 0x10, 0xe8, 0x85, 0x5a, 0xce, 0x68, 0x2d, 0x18, 0x43, 0xd8, 0xcb, 0x12, 0x8f, 0xe4],
        ( 9,"ik_sch")   =>  vec![0xd6, 0xaa, 0x74, 0xfd, 0xd2, 0xaf, 0x72, 0xfa, 0xda, 0xa6, 0x78, 0xf1, 0xd6, 0xab, 0x76, 0xfe],
        ( 9,"ik_add")   =>  vec![0x5f, 0x72, 0x64, 0x15, 0x57, 0xf5, 0xbc, 0x92, 0xf7, 0xbe, 0x3b, 0x29, 0x1d, 0xb9, 0xf9, 0x1a],
        (10,"istart")   =>  vec![0x63, 0x53, 0xe0, 0x8c, 0x09, 0x60, 0xe1, 0x04, 0xcd, 0x70, 0xb7, 0x51, 0xba, 0xca, 0xd0, 0xe7],
        (10,"is_row")   =>  vec![0x63, 0xca, 0xb7, 0x04, 0x09, 0x53, 0xd0, 0x51, 0xcd, 0x60, 0xe0, 0xe7, 0xba, 0x70, 0xe1, 0x8c],
        (10,"is_box")   =>  vec![0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0],
        (10,"ik_sch")   =>  vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f],
        (10,"ioutput")  => vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff],
        _               => panic!("error in lookup for aes inv cipher check"),
    }
}