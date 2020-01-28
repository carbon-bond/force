import * as React from 'react';
import { render } from 'react-dom';

import Konva from 'konva';

const CATEGORY_SIZE = 80;
class CategoryShape {
	entity: Konva.Group
	layer: Konva.Layer
	constructor(layer: Konva.Layer, x: number, y: number) {
		this.layer = layer;
		this.entity = new Konva.Group({
			x: x - CATEGORY_SIZE/2,
			y: y - CATEGORY_SIZE/2,
		});
		const rect = new Konva.Rect({
			width: CATEGORY_SIZE,
			height: CATEGORY_SIZE,
			cornerRadius: 5,
			draggable: true,
			stroke: 'black',
			strokeWidth: 3,
		});
		rect.on('dragstart', () => {
			rect.stopDrag();
			this.entity.startDrag();
		});
		this.entity.on('mouseenter', () => {
			this.add_arrow();
		})
		this.entity.on('mouseleave', () => {
			this.remove_arrow();
		})
		this.entity.add(rect);
		this.layer.add(this.entity);
		this.add_arrow();
		// this.layer.draw();
	}
	add_arrow(): void {
		if (this.entity.find('Arrow').length > 0) {
			return;
		}
		const center_x = CATEGORY_SIZE / 2;
		const center_y = CATEGORY_SIZE / 2;
		let positions = [
			[1, 0],
			[-1, 0],
			[0, 1],
			[0, -1],
		];
		for (let p of positions) {
			let start_x = center_x + p[0]*CATEGORY_SIZE/2;
			let start_y = center_y + p[1]*CATEGORY_SIZE/2;
			const length = 25;
			let end_x = start_x + p[0]*length;
			let end_y = start_y + p[1]*length;
			const arrow = new Konva.Arrow({
				stroke: '#baf',
				fill: '#baf',
				strokeWidth: 15,
				hitStrokeWidth: 15,
				opacity: 0.5,
				points: [
					start_x, start_y,
					(start_x + end_x) / 2, (start_y + end_y) / 2,
					end_x, end_y,
				]
			});
			this.entity.add(arrow);
		}
		this.layer.draw();
	}
	remove_arrow(): void {
		this.entity.find('Arrow').each(a => a.destroy());
		this.layer.draw();
	}
}

function canvas() {
	// first we need to create a stage
	const stage = new Konva.Stage({
		container: 'canvas',   // id of container <div>
		width: 600,
		height: 600
	});

	// then create layer
	const layer = new Konva.Layer();

	stage.on('dblclick dbltap', function () {
		let position = stage.getPointerPosition();
		const category = new CategoryShape(layer, position.x, position.y);
		layer.add(category.entity);
		layer.draw();
	});

	// add the layer to the stage
	stage.add(layer);

	// draw the image
	layer.draw();

}

canvas();

function App() {
	return <div> 看板規則設定 </div>;
}

render(<App />, document.getElementById('app'));