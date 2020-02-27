fn main() {
    //    let proto_path=&["src/helloworld"];
    //    let proto_dir = proto_path
    //        .parent()
    //        .expect("proto file should reside in a directory");

    //    tonic_build::configure()
    //        .out_dir("src/proto/helloworld")
    //        .compile(
    //            &["src/proto/helloworld/helloworld.proto"],
    //            &["src/proto/helloworld"],
    //        )
    //        .unwrap();
    tonic_build::compile_protos("src/proto/helloworld/helloworld.proto").unwrap()
    /*
    tonic_build::compile_protos("src/proto/routeguide/route_guide.proto").unwrap();
    tonic_build::compile_protos("src/proto/echo/echo.proto").unwrap();
    tonic_build::compile_protos("src/proto/google/pubsub/pubsub.proto").unwrap();
    */
}
