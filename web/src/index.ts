async function delay(ms: number): Promise<void> {
    return new Promise(ok => setTimeout(ok, ms));
}

async function main() {
    let container = document.querySelector("#container");
    if (container == null) {
        console.error("Container not found");
        return;
    }
    await delay(1000);
    container.innerHTML = "Hello from script!";
    await delay(1000);
    container.innerHTML = "One second later!"
    let res = await fetch("mail/firefox/%3C0101016f050098ac-27b8d337-aa46-4212-967e-59c11a4ee445-000000%40us-west-2.amazonses.com%3E/ROOT/0/0.txt");
    let text = await res.text();
    container.innerHTML = "Got this from server: <br><br>" + text;
}

main();