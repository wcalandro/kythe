var a = [true,false];
function foo(x) { }

for (var i=0;i<3;i++) {
    foo(a[i]);
}
for (var k in a) {
    foo(a[k]);
}

var b = (null : ?{[key: string]: string});
for (var j in b) {
    foo(b[j]);
}

var c;
for (var m in (c = b)) {
    foo(c[m]);
}

var d;
for (var n in (d = a)) {
    foo(d[n]);
}

for (var x in null) {
    foo(x); // unreachable
}
