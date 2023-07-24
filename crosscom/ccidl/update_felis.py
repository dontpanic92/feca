import os
import shutil

os.system("python3.10 main.py idl/felis.idl felis::comdef")
shutil.copyfile("test.rs", "../../felis/src/comdef.rs")
