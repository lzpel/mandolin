import "@typespec/http";

using Http;
@service(#{ title: "WebFaceSimulator" })
namespace DemoService;

@format("uuid")
scalar UUID extends string;

@error
model Error {
  code: int32;
  message: string;
}

model Task {
  id: UUID;
}
model TaskRequest{
  ...Task;
  analysis: string;
}

@route("/tasks")
interface Tasks {
  @get list(): Task[] | Error;
  @post push(@body request: TaskRequest): Task | Error;
  @delete pop(@path id: UUID): Task | Error;
}
