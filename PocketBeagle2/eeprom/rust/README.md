# EEPROM Example

Read and parse EEPROM contents in Rust.

# Usage

```console
debian@pocketbeagle2:~/vsx-examples/PocketBeagle2/eeprom/rust$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/eeprom`
BeagleEeprom {
    magic_number: [
        170,
        85,
        51,
        238,
    ],
    hdr1: Header {
        id: 1,
        len: 55,
    },
    hdr2: Header {
        id: 16,
        len: 46,
    },
    board_info: BoardInfo {
        name: "POCKETBEAGL2A00G",
        version: "A0",
        proc_number: "0000",
        variant: "0G",
        pcb_revision: "0G",
        schematic_bom_revision: "A0",
        software_revision: "00",
        vendor_id: "01",
        build_week: "02",
        build_year: "25",
        serial: "PB20000183",
    },
    hdr3: Header {
        id: 17,
        len: 2,
    },
    ddr_info: 4776,
    termination: 254,
}
```
