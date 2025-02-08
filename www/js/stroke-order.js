export class StrokeOrder extends HTMLElement {
  constructor() {
    super();
    this.addEventListener('click', this.onClick.bind(this));
    this.render();
  }

  attributeChangedCallback(name, _oldValue, _newValue) {
    console.error('TODO handle change', name);
  }

  onClick() {
    this.writer.animateCharacter();
  }

  render() {
    const character = this.getAttribute('character');
    const dataPath = this.getAttribute('data-path');
    this.writer = HanziWriter.create(
      this,
      character,
      {
        charDataLoader: (ch, onComplete) => fetch(`${dataPath}/${character}.json`)
          .then(r => r.json())
          .then(onComplete),
        width: 100,
        height: 100,
        strokeAnimationSpeed: 2,
        padding: 5,
        delayBetweenStrokes: 100,
        delayBetweenLoops: 500
      }
    );
    this.writer.animateCharacter();
  }
}

self.customElements.define('stroke-order', StrokeOrder);
