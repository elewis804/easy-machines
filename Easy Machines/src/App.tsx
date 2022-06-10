import type { Component } from 'solid-js';

const App: Component = () => {

  const onButtonClick = () => {
    console.log("Button clicked.");
  }

  return (
    <div>
      <header class="text-4xl text-green-700 text-center py-20">This app is very poggers!</header>
      <div class="flex items-center grid grid-cols-5 gap-5">
        <div class="text-center ">1</div>
        <div class="text-center ">2</div>
        <div class="text-center ">3</div>
        <div class="text-center ">4</div>
        <div class="text-center ">5</div>
      </div>
      <div></div>
      <div>
        <button onClick={onButtonClick}>Click me</button>
      </div>
    </div>
  );
};

export default App;