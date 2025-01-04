"use client"
import React from "react";
import {example} from "@/lib";
export default function Compare(props: {openapi: string}) {
	const [text, setText] = React.useState(props.openapi)
	const textareaStyle: React.CSSProperties = { flex: 1, width: '50%', height: '100%', boxSizing: 'border-box', border: '1px solid #ccc', padding: '10px', resize: "none"};
	return (<div style={{flexGrow: 1, display: "flex"}}>
				<textarea style={textareaStyle} value={text} onChange={v=>setText(v.currentTarget.value)}/>
				<textarea style={textareaStyle} value={example(text)} readOnly={true}/>
			</div>
	)
}