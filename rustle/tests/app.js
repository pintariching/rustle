export default function() {
    let button_1;
    let txt_2;
    let h1_3;
    let txt_4;
    let txt_5;
    let txt_6;
    let button_7;
    let txt_8;
    let double;

    let collectChanges = [];
    let updateCalled = false;
    function update(changed) {
        changed.forEach((c) => collectChanges.push(c));
        if (updateCalled) return;
        updateCalled = true;

        updateReactiveDeclarations(collectChanges);
        if (typeof lifecycle !== "undefined") lifecycle.update(collectChanges);

        collectChanges = [];
        updateCalled = false;
    }

let counter = 1;
const increment = ()=>(counter++, update([
        "counter"
    ]));
const decrement = ()=>(counter--, update([
        "counter"
    ]));


    update(["counter", "double"]);

    function updateReactiveDeclarations() {
        if (["counter"].some(name => collectChanges.includes(name))) {
            double = counter * 2;
            update(["double"]);
        }
    }

    var lifecycle = {
        create(target) {
            button_1 = document.createElement('button');
            button_1.setAttribute('class', 'button rustle-jT3LlVD');
            button_1.addEventListener('click', increment);
            txt_2 = document.createTextNode('Increment');
            button_1.appendChild(txt_2);
            target.appendChild(button_1);
            h1_3 = document.createElement('h1');
            h1_3.setAttribute('class', 'text rustle-jT3LlVD');
            txt_4 = document.createTextNode(counter);
            h1_3.appendChild(txt_4);
            txt_5 = document.createTextNode('* 2 =');
            h1_3.appendChild(txt_5);
            txt_6 = document.createTextNode(double);
            h1_3.appendChild(txt_6);
            target.appendChild(h1_3);
            button_7 = document.createElement('button');
            button_7.setAttribute('class', 'button rustle-jT3LlVD');
            button_7.addEventListener('click', decrement);
            txt_8 = document.createTextNode('Decrement');
            button_7.appendChild(txt_8);
            target.appendChild(button_7);
        },
        update(changed) {
            if (changed.includes("counter")) {
                txt_4.data = counter;
            }
            if (changed.includes("double")) {
                txt_6.data = double;
            }
        },
        destroy() {
            button_1.removeEventListener('click', increment);
            target.removeChild(button_1);
            target.removeChild(h1_3);
            button_7.removeEventListener('click', decrement);
            target.removeChild(button_7);
        },
    };
    return lifecycle;
}