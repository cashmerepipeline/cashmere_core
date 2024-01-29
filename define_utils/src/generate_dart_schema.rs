use convert_case::{Case, Casing};
use manage_define::general_field_names;

/// zh: 生成isar本地数据库的schema dart代码
pub fn generate_dart_schema_code(
    manage_name: &str,
    manage_schema: &Vec<(String, i32, String)>,
    package_name: &str,
) -> String {
    let import_statements = vec![
        "import 'package:objectbox/objectbox.dart';\n",
    ];

    let part_statement = format!("import 'package:{}/objectbox.g.dart';\n\n", package_name);

    let class_name = manage_name.to_case(Case::Pascal);
    let general_properties = r#"
    // objectbox id
    @Id()
    int bid;
    @Index()
    String? oid;
    @Index()
    final String nid;
    String? creator;
    // millisceconds utc
    @Property(type: PropertyType.dateNano) 
    DateTime? createTimestamp;
    String? modifier;
    @Property(type: PropertyType.dateNano) 
    DateTime? modifyTimestamp;
    String? owner;
    List<String>? groups;
    List<String>? comments;
    List<String>? tags;
    String? description;
    bool? removed;
    // millisceconds utc
    @Property(type: PropertyType.date) 
    DateTime? syncTime;
    "#;

    let general_param_properties = r#"this.bid = 0,
      required this.nid,
"#;

    let mut define_statements = vec![];
    let mut construct_statements = vec![];
    for (s_name, s_id, s_type) in manage_schema.iter() {
        let s = format!(
            "{}? {};\n",
            get_dart_data_type(s_type),
            s_name.to_case(Case::Camel)
        );
        define_statements.push(s);

        let cs = format!(
            "required this.{},\n",
            s_name.to_case(Case::Camel)
        );
        construct_statements.push(cs);
    }

    let class_define_statement = format!(
        "{}{}
@Entity()\nclass {name}{{
      {gprop}
      {mprop}
    {name}({{
      {gparam}
    }});
}}",
        import_statements.join(""),
        part_statement,
        name = class_name,
        gprop = general_properties,
        mprop = define_statements.join("      "),
        gparam = general_param_properties,
        // cprop = construct_statements.join("      "),
    );

    class_define_statement
}

/// bson类型字符串转换为dart类型字符串映射
pub fn get_dart_data_type(field_type: &str) -> &str {
    match field_type {
        "String" => "String",
        "Int32" => "int",
        "Int64" => "int",
        "Boolean" => "bool",
        "Double" => "double",
        "DateTime" => "DateTime",
        "Timestamp" => "DateTime",
        "Array<Int32>" => "List<int>",
        "Array<Int64>" => "List<int>",
        "Array<Double>" => "List<double>",
        "Array<bool>" => "List<bool>",
        "Array<String>" => "List<String>",
        "Array<Document>" => "List<Map<String, dynamic>",
        "Array" => "List",
        "Document" => "Map<String, dynamic>",
        _ => "None",
    }
}
