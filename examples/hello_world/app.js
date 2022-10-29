export default function () {
    let name = "world";

    let h1_1;
    let txt_2;
    let txt_3;
    let txt_4;
    const lifecycle = {
        create(target) {
            h1_1 = document.createElement("h1");
            txt_2 = document.createTextNode("Hello ");
            h1_1.appendChild(txt_2);
            txt_3 = document.createTextNode(name);
            h1_1.appendChild(txt_3);
            txt_4 = document.createTextNode("!");
            h1_1.appendChild(txt_4);
            target.appendChild(h1_1);
        },
        update(changed) {},
        destroy() {
            target.removeChild(h1_1);
        },
    };
    return lifecycle;
}
