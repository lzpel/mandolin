"use client"
import React from "react";
import {example, templates} from "@/out";

export default function Compare(props: { files: { [key: string]: string } }) {
	const [text, setText] = React.useState(props.files["openapi_petstore.yaml"])
	const [selectedTemplate, setSelectedTemplate] = React.useState<string>("RUST_AXUM");

	const textareaStyle: React.CSSProperties = {
		flexGrow: 1,
		height: '100%',
		boxSizing: 'border-box',
		border: '1px solid #ccc',
		padding: '10px',
		resize: "none"
	};
	const preset=Object.entries(props.files).map(([key, value]) => <button key={key} onClick={() => setText(value)}>{key}</button>)
	const framework=templates().map(v=><label key={v}><input type="radio" name="template" value="RUST_AXUM" required checked={selectedTemplate===v} onChange={()=>setSelectedTemplate(v)}/>{v}</label>)
	return <>
		<span>preset openapi: {preset}</span>
		<span>target framework:{framework}</span>
		<div style={{flexGrow: 1, display: "flex", alignItems: "center"}}>
			<textarea style={textareaStyle} value={text} onChange={v => setText(v.currentTarget.value)}/>
			<div style={{fontSize: "xxx-large"}}>→</div>
			<textarea style={textareaStyle} value={example(text, selectedTemplate)} readOnly={true}/>
		</div>
	</>
}