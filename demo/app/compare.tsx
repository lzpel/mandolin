"use client"
import React from "react";
import {example} from "@/lib";
export default function Compare(props: {openapi: string}) {
	const [text, setText] = React.useState(props.openapi)
	const textareaStyle: React.CSSProperties = { flexGrow: 1, height: '100%', boxSizing: 'border-box', border: '1px solid #ccc', padding: '10px', resize: "none"};
	return (<div style={{flexGrow: 1, display: "flex", alignItems: "center"}}>
				<textarea style={textareaStyle} value={text} onChange={v=>setText(v.currentTarget.value)}/>
				<div style={{fontSize: "xxx-large"}}>→</div>
				<textarea style={textareaStyle} value={example(text)} readOnly={true}/>
			</div>
	)
}