import("../pkg").then(module => {
    const numbers = module.generate_numbers();
    document.getElementById("code").innerHTML = `De code van vandaag is:\n\n${numbers}`;
});
