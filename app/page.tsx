import { Greet, ButtonClick } from './render'
'use client';

export default function Home() {
  return (
    <main>
      <CreateTextDisplay />

      <br/>
      <hr/>
      <br/>

      <CreateInput />
    </main>
  )
}

function CreateTextDisplay() {
  return (
    <div className="Center Vertical">
        <div>
          <h1>Read Below!</h1>
        </div>
        <div className="Content">
          <h1 id="changing-text">Default</h1>
        </div>
      </div>
  )
}

function CreateInput() {
  return (
    <div className="Center Vertical">
      <label>Input Text</label>
      <input onChange={(e) => {console.log("@todo")}} defaultValue="Sample Text" />

      <br/>

      <label>Reading Speed</label>
      <input onChange={(e) => {console.log("@todo")}} type="number" defaultValue="200" />
    </div>
  )
}
