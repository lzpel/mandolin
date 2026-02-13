"use client"
import React from "react";
import {example, templates} from "@/out";
import Editor from '@monaco-editor/react';

export default function Compare(props: { files: { [key: string]: string } }) {
	const [text, setText] = React.useState(props.files["openapi_petstore.yaml"])
	const [selectedTemplate, setSelectedTemplate] = React.useState<string>("RUST_AXUM");

	const textareaStyle: React.CSSProperties = {
		flexGrow: 1,
		height: '100%',
		boxSizing: 'border-box',
		border: '1px solid #ccc',
	};
	const preset=Object.entries(props.files).map(([key, value]) => <button key={key} onClick={() => setText(value)}>{key}</button>)
	const framework=templates().map(v=><label key={v}><input type="radio" name="template" value="RUST_AXUM" required checked={selectedTemplate===v} onChange={()=>setSelectedTemplate(v)}/>{v}</label>)
	return <>
		<span>preset openapi: {preset}</span>
		<span>target framework:{framework}</span>
		<div style={{flexGrow: 1, display: "flex", alignItems: "center"}}>
			<div style={textareaStyle}>
				<Editor
					defaultLanguage={"yaml"}
					value={text}
					onChange={v => setText(v||"")}
				/>
			</div>
			<div style={{fontSize: "xxx-large"}}>→</div>
			<div style={textareaStyle}>
				<Editor
					defaultLanguage={selectedTemplate.includes("RUST") ? "rust": "typescript"}
					value={example(text, selectedTemplate)}
				/>
			</div>
		</div>
	</>
}