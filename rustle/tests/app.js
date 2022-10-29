export default function () {
    let x = 0;
    let y = 0;
    const handleMousemove = (event) => {
        x = event.clientX;
        y = event.clientY;
        lifecycle.update(["x"]);
        lifecycle.update(["y"]);
    };

    let div_1;
    let txt_2;
    let txt_3;
    let txt_4;
    let txt_5;
    const lifecycle = {
        create(target) {
            div_1 = document.createElement("div");
            div_1.addEventListener("mousemove", handleMousemove);
            div_1.setAttribute("class", "full");
            txt_2 = document.createTextNode("The mouse position is x:");
            div_1.appendChild(txt_2);
            txt_3 = document.createTextNode(x);
            div_1.appendChild(txt_3);
            txt_4 = document.createTextNode("y:");
            div_1.appendChild(txt_4);
            txt_5 = document.createTextNode(y);
            div_1.appendChild(txt_5);
            target.appendChild(div_1);
        },
        update(changed) {
            if (changed.includes("x")) {
                txt_3.data = x;
            }

            if (changed.includes("y")) {
                txt_5.data = y;
            }
        },
        destroy() {
            div_1.removeEventListener("mousemove", handleMousemove);
            target.removeChild(div_1);
        },
    };
    return lifecycle;
}
