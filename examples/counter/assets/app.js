
	export default function() {
		let counter = 1665844550;
const increment = ()=>(counter++, lifecycle.update([
        "counter"
    ]));
const decrement = ()=>(counter--, lifecycle.update([
        "counter"
    ]));

		let button_1;
let txt_2;
let div_3;
let txt_4;
let button_5;
let txt_6;
		const lifecycle = {
			create(target) {
				button_1 = document.createElement('button');
button_1.addEventListener('click', increment);
txt_2 = document.createTextNode('Increment');
button_1.appendChild(txt_2);
target.appendChild(button_1);
div_3 = document.createElement('div');
txt_4 = document.createTextNode(counter);
div_3.appendChild(txt_4);
target.appendChild(div_3);
button_5 = document.createElement('button');
button_5.addEventListener('click', decrement);
txt_6 = document.createTextNode('Decrement');
button_5.appendChild(txt_6);
target.appendChild(button_5);
			},
			update(changed) {
				
					if (changed.includes('counter')) {
						txt_4.data = counter;
					}
				
			},
			destroy() {
				button_1.removeEventListener('click', increment);
target.removeChild(button_1);
target.removeChild(div_3);
button_5.removeEventListener('click', decrement);
target.removeChild(button_5);
			},
		};
		return lifecycle;
	}
	