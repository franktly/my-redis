use std::collections::BTreeMap;
use tracing_subscriber::Layer;

pub struct CustomLayer;

#[derive(Debug)]
struct CustomFieldStorage(BTreeMap<String, serde_json::Value>);

impl<S> Layer<S> for CustomLayer 
where 
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>
{
    // event fired when `info!, debug!, error! ...` called
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        ctx:   tracing_subscriber::layer::Context<'_, S>)
    {
        println!("********** Origin Begin **********");
        println!("Got event!");
        println!("level = {:?}", event.metadata().level());
        println!("target = {:?}", event.metadata().target());
        println!("name = {:?}", event.metadata().name());

        println!("\r\n");
        for field in event.fields(){
            println!("field= {}", field.name());
        }
        println!("********** Origin End **********");

        println!("\r\n");
        println!("********** Print Visitor Begin **********");
        let mut visitor = PrintlnVisitor;
        event.record(&mut visitor);
        println!("********** Print Visitor End **********");

        println!("\r\n");
        println!("********** Json Visitor Begin **********");
        // convert the values into a JSON object
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        // output the event in JSON
        let output = serde_json::json!({
            "level":  format!("{:?}", event.metadata().level()),
            "target": event.metadata().target(),
            "name":   event.metadata().name(),
            "fields": fields,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());

        println!("********** Json Visitor End **********");

        println!("\r\n");
        println!("********** Span Begin **********");
        // parent span
        let parent_span = ctx.event_span(event).unwrap();
        println!("parent span");
        println!("name = {}", parent_span.name());
        println!("target = {}", parent_span.metadata().target());
        println!();

        // traverse all spans in specified scope
        let scope = ctx.event_scope(event).unwrap();
        println!("********** Origin Traverse Begin **********");
        for span in scope{
            println!("an ancestor span");
            println!("name = {}", span.name());
            println!("target = {}", span.metadata().target());
            // get extensions by span(correspond to on_new_span event save extension by span)
            let extensions = span.extensions();
            let storage = extensions.get::<CustomFieldStorage>().unwrap();
            println!("extension storage span");
            println!("name = {}", span.metadata().name());
            println!("target = {}", span.metadata().target());
            println!("stored fields = {:?}", storage);
        }
        println!("********** Origin Traverse End **********");
        /*
           println!("********** Inverse Traverse Begin **********");
           for span in scope.from_root(){
           println!("an ancestor span");
           println!("name = {}", span.name());
           println!("target = {}", span.metadata().target());
           }
           println!("********** Inverse Traverse End **********");*/

        println!("********** Span End **********");
    }

    // record data when span create
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id:    &tracing::span::Id,
        ctx:   tracing_subscriber::layer::Context<'_,S>) 
    {
        let span = ctx.span(id).unwrap();

        // json visitor and storage
        // construct a JSON object and storage 
        let mut fields =  BTreeMap::new();
        let mut json_visitor = JsonVisitor(&mut fields);
        attrs.record(&mut json_visitor);

        let storage = CustomFieldStorage(fields);
        // get extension by span and storage span data
        let mut extensions = span.extensions_mut();
        extensions.insert::<CustomFieldStorage>(storage);

        // print visitor
        println!("Got on_new_span");
        println!("level = {:?}", span.metadata().level());
        println!("target = {:?}", span.metadata().target());
        println!("name = {:?}", span.metadata().name());

        let mut print_visitor = PrintlnVisitor;
        attrs.record(&mut print_visitor);
    }

    // record data after span create
    fn on_record(
        &self,
        id:    &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx:   tracing_subscriber::layer::Context<'_, S>)
    {
        let span = ctx.span(id).unwrap();

        let mut  extensions = span.extensions_mut();
        let custom_field_storage: &mut CustomFieldStorage = 
            extensions.get_mut::<CustomFieldStorage>().unwrap();
        let json_data: &mut BTreeMap<String, serde_json::Value> = 
            &mut custom_field_storage.0;

        let mut visitor = JsonVisitor(json_data);
        values.record(&mut visitor);
    }
}

// implement Visit trait for custom print visitor and override record_X method
pub struct PrintlnVisitor;

impl tracing::field::Visit for PrintlnVisitor{
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64){
        println!("field = {}, value = {}", field.name(), value);
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64){
        println!("field = {}, value = {}", field.name(), value);
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64){
        println!("field = {}, value = {}", field.name(), value);
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool){
        println!("field = {}, value = {}", field.name(), value);
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str){
        println!("field = {}, value = {}", field.name(), value);
    }

    fn record_error(&mut self, field: &tracing::field::Field, 
        value: &(dyn std::error::Error + 'static)){
        println!("field = {}, value = {}", field.name(), value);
    }

    fn record_debug(&mut self, field: &tracing::field::Field, 
        value: &(dyn std::fmt::Debug)){
        println!("field = {}, value = {:?}", field.name(), value);
    }
}

// implement Visit trait for custom json visitor and override record_X method
struct JsonVisitor<'a>(&'a mut BTreeMap<String, serde_json::Value>);

impl<'a> tracing::field::Visit for JsonVisitor<'a>{
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64){
        self.0.insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64){
        self.0.insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64){
        self.0.insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool){
        self.0.insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str){
        self.0.insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_error(&mut self, field: &tracing::field::Field, 
        value: &(dyn std::error::Error + 'static)){
        self.0.insert(field.name().to_string(), serde_json::json!(value.to_string()));
    }

    fn record_debug(&mut self, field: &tracing::field::Field, 
        value: &(dyn std::fmt::Debug)){
        self.0.insert(field.name().to_string(), serde_json::json!(format!("{:?}", value)));
    }
}
