function a() {
    function b() {
        console.log(c);
    }
    const c = 5;
    b();
}

a()