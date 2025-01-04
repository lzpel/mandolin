import type {Metadata} from "next";

export const metadata: Metadata = {
	title: "mandolin",
	description: "Generate openapi-based server code",
};

export default function RootLayout(props: Readonly<{ children: React.ReactNode }>) {
    const full={margin: 0, height: "100%"}
	return (
		<html lang="jp" style={full}>
		<body style={{...full, display: "flex", flexFlow: "column"}}>
		{props.children}
		</body>
		</html>
	);
}
