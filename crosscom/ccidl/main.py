import gen_rust
import parser

content = open("idl/felis.idl", encoding="utf-8").read()
unit = parser.parse(content)
print(unit)

open("test.rs", "w").write(gen_rust.RustGen(unit, 'crate::defs').gen())
