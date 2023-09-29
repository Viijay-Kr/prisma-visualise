export interface SchemaResult {
  result: Array<{
    id: string;
    name: string;
    fields: Array<{
      name: string;
      type: string;
      is_index: boolean;
      constraints: string[];
      relation_ship_fields: string[];
      relation_ship_references: string[];
    }>;
    code: string;
  }>;
}
