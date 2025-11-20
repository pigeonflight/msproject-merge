# MPP Format Research: Building a Rust-Based Reader/Writer

## Executive Summary

The MPP (Microsoft Project) file format is a proprietary binary format used by Microsoft Project to store project management data. **Microsoft does not publicly release specifications** for the MPP format, making it challenging to develop custom readers or writers. Building a native Rust implementation would require significant reverse-engineering effort.

## MPP Format Overview

### Format Characteristics

1. **Proprietary Binary Format**: MPP files are binary files with no public specification
2. **OLE2 Compound Document Structure**: MPP files use the OLE2 (Object Linking and Embedding) compound document format, similar to older Office formats
3. **Version-Specific Variations**: The format has evolved across different Microsoft Project versions:
   - **MPP8**: Microsoft Project 98
   - **MPP9**: Microsoft Project 2000 and 2002
   - **MPP12**: Microsoft Project 2003 and 2007
   - **MPP14**: Microsoft Project 2010, 2013, 2016, 2019, and later

### Internal Structure

MPP files contain multiple internal streams within the OLE2 compound document structure:
- Project summary information
- Task data
- Resource data
- Assignment data
- Calendar information
- Custom fields
- Views and formatting information
- Relationships and dependencies

## Challenges in Building a Rust MPP Reader/Writer

### 1. Lack of Official Documentation
- No public specification available from Microsoft
- Must rely on reverse-engineering or third-party research
- Format may change between versions without notice

### 2. Complex Binary Structure
- OLE2 compound document format requires parsing multiple streams
- Binary encoding of various data types (dates, durations, relationships)
- Compressed or encoded data sections
- Version-specific format differences

### 3. Data Complexity
- Task hierarchies and dependencies
- Resource assignments and calendars
- Custom fields and formulas
- Views, filters, and formatting
- Baseline data and tracking information

### 4. Testing and Validation
- Need access to MPP files from various Project versions
- Must ensure compatibility across versions
- Difficult to validate correctness without official reference

## Existing Solutions and Approaches

### 1. MPXJ Library (Java)
- **Status**: Most mature third-party solution
- **Capabilities**: 
  - Reads MPP files from Project 98 through Project 2019
  - Supports multiple project file formats (MPP, MPX, MSPDI, XML, etc.)
  - Can write to various formats (though MPP writing support is limited)
- **Integration Options**:
  - Use JNI (Java Native Interface) bindings from Rust
  - Requires JVM dependency
  - Adds complexity and runtime overhead

### 2. Alternative Formats
- **MSPDI (Microsoft Project Data Interchange)**: XML-based format, publicly documented
- **MPX (Microsoft Project Exchange)**: Text-based format, easier to parse
- **XML Export**: Microsoft Project can export to XML format

## Technical Requirements for Rust Implementation

### Core Dependencies Needed

1. **OLE2 Compound Document Parser**
   - Rust crate: `ole2`, `cfb`, or `mscfb` (or custom implementation)
   - Need to parse compound document structure
   - Extract streams from the OLE2 container
   - Handle stream names and directory structure

2. **Binary Data Parsing**
   - `byteorder` for endianness handling (little-endian for MPP)
   - `nom` or `binread` for declarative binary parsing
   - Custom parsers for MPP-specific data structures
   - Date/time parsing (MPP uses specific date encodings, often as days since 1899-12-30)

3. **Data Structures**
   - Task, Resource, Assignment models
   - Relationship/dependency tracking
   - Calendar and working time logic
   - Custom fields and formulas
   - Views and formatting

### Key Data Structures to Parse

#### Tasks
- Task ID, Name, Duration
- Start/Finish dates
- Percent Complete, Status
- Priority, Work, Cost
- Predecessors/Successors (dependencies)
- Resource assignments
- Outline level (hierarchy)
- WBS code, Notes

#### Resources
- Resource ID, Name, Type
- Initials, Group
- Max Units, Standard Rate, Overtime Rate
- Calendar assignments
- Cost per use

#### Assignments
- Task ID, Resource ID
- Work, Units, Cost
- Start/Finish dates
- Remaining work

#### Calendars
- Base calendar definitions
- Working days/times
- Exceptions (holidays, etc.)
- Calendar assignments to tasks/resources

#### Project Information
- Project start/end dates
- Calendar settings
- Currency, language
- Custom properties

### Implementation Phases

#### Phase 1: OLE2 Document Parsing
- Parse OLE2 compound document structure
- Extract and identify internal streams
- Map stream names to data types

#### Phase 2: Stream Parsing
- Reverse-engineer stream formats
- Parse binary data structures
- Handle version-specific differences

#### Phase 3: Data Model Construction
- Build Rust data structures for:
  - Tasks (name, dates, duration, status, etc.)
  - Resources (name, type, availability, etc.)
  - Assignments (task-resource relationships)
  - Calendars and working times
  - Dependencies and relationships

#### Phase 4: Writer Implementation
- Generate OLE2 compound document structure
- Serialize data to binary streams
- Handle version-specific encoding

#### Phase 5: Testing and Validation
- Test with MPP files from various Project versions
- Validate data integrity
- Handle edge cases and error conditions

## Estimated Effort

### Reading Implementation
- **OLE2 parsing**: 2-4 weeks
- **Stream reverse-engineering**: 4-8 weeks (per version)
- **Data model and parsing**: 4-6 weeks
- **Testing and refinement**: 4-6 weeks
- **Total**: ~14-24 weeks for basic reading support

### Writing Implementation
- **OLE2 document generation**: 2-3 weeks
- **Stream serialization**: 6-10 weeks (per version)
- **Data encoding**: 4-6 weeks
- **Testing and validation**: 4-6 weeks
- **Total**: ~16-25 weeks for basic writing support

**Note**: These estimates assume significant reverse-engineering work and may vary based on:
- Access to sample MPP files
- Existing research/documentation available
- Complexity of features to support

## Recommended Approach

### Short-Term Solution
1. **Use MPXJ via JNI**: Create Rust bindings to MPXJ library
   - Pros: Fastest path to MPP support, proven library
   - Cons: Requires JVM, adds complexity

2. **Support Alternative Formats**: Focus on MSPDI (XML) and MPX formats
   - Pros: Easier to implement, well-documented
   - Cons: Requires users to export/convert files

### Long-Term Solution
1. **Incremental Native Implementation**:
   - Start with reading support for one MPP version
   - Gradually add support for other versions
   - Add writing support after reading is stable

2. **Community Collaboration**:
   - Open-source the project
   - Collaborate with others working on MPP format
   - Share reverse-engineering findings

## Rust Ecosystem Considerations

### Existing Crates

#### OLE2/Compound Document Parsing
- **`ole`**: Basic OLE2 compound document reader (may be outdated)
- **`cfb`**: Compound File Binary Format parser (alternative to OLE2)
- **`mscfb`**: Microsoft Compound File Binary Format parser
- **Note**: May need to implement custom OLE2 parser or contribute to existing crates

#### Binary Data Parsing
- **`byteorder`**: Endianness handling for binary data
- **`nom`**: Parser combinator library for binary formats
- **`binread`**: Declarative binary file parsing

#### Date/Time Handling
- **`chrono`**: Already in use, handles date/time parsing
- **`time`**: Alternative date/time library

#### XML Parsing (for MSPDI support)
- **`quick-xml`**: Fast XML parser
- **`xml-rs`**: XML parser with DOM support
- **`roxmltree`**: Read-only XML tree parser

#### Other Useful Crates
- **`thiserror`**: Error handling
- **`serde`**: Already in use, for serialization
- **`anyhow`**: Error context handling

### Missing Pieces
- No existing Rust MPP parser/writer
- Limited OLE2 support in Rust ecosystem (may need custom implementation)
- Would need to build most parsing logic from scratch
- No existing reverse-engineering documentation in Rust ecosystem

### Potential Implementation Structure

```rust
// Example crate structure
mpp-rs/
├── src/
│   ├── lib.rs
│   ├── ole2.rs          // OLE2 compound document parser
│   ├── mpp8.rs          // MPP8 (Project 98) format
│   ├── mpp9.rs          // MPP9 (Project 2000/2002) format
│   ├── mpp12.rs         // MPP12 (Project 2003/2007) format
│   ├── mpp14.rs         // MPP14 (Project 2010+) format
│   ├── models.rs        // Data models (Task, Resource, etc.)
│   ├── reader.rs        // High-level reader API
│   └── writer.rs        // High-level writer API
└── tests/
    └── samples/         // Test MPP files (various versions)
```

## Reverse-Engineering Strategies

### Tools and Techniques

1. **Hex Editors and Binary Analysis**
   - Use hex editors (e.g., `hexdump`, `xxd`, or GUI tools)
   - Compare MPP files with known data to identify patterns
   - Look for ASCII strings (task names, resource names) to locate data sections

2. **OLE2 Stream Extraction**
   - Use tools like `olefile` (Python) or similar to extract streams
   - Analyze stream names and contents
   - Map stream names to data types

3. **Version Comparison**
   - Create simple projects in different Project versions
   - Compare binary structures between versions
   - Identify version-specific differences

4. **Reference Implementation Analysis**
   - Study MPXJ source code (Java) for insights
   - Understand parsing logic and data structures
   - Adapt algorithms to Rust

5. **Incremental Testing**
   - Start with simplest possible MPP files
   - Gradually add complexity (tasks, resources, dependencies)
   - Use test-driven development approach

### Resources for Reverse Engineering

1. **MPXJ Source Code**: Available on GitHub, provides reference implementation
2. **OLE2/CFB Format Documentation**: Publicly documented format
3. **Microsoft Project XML Schema**: MSPDI format can provide insights into data model
4. **Community Forums**: Reverse engineering communities may have insights
5. **Binary Format Analysis Tools**: 
   - `010 Editor` with templates
   - `HxD` hex editor
   - Python `olefile` library for stream extraction

## Legal and Ethical Considerations

1. **Reverse Engineering**: Generally legal for interoperability purposes (DMCA safe harbor, EU Software Directive), but check local laws
2. **Patent Issues**: Microsoft may have patents on MPP format - research existing patents
3. **Compatibility Testing**: Need legitimate MPP files for testing (create test files or use sample projects)
4. **Open Source**: Consider licensing implications if open-sourcing the project
5. **Documentation**: Document reverse-engineering findings to help others

## Conclusion

Building a native Rust MPP reader/writer is **technically feasible but requires significant effort**:

- **Reading**: 14-24 weeks of development
- **Writing**: 16-25 weeks of development
- **Maintenance**: Ongoing effort to support new Project versions

**Recommended Path Forward**:
1. **Immediate**: Support MSPDI/XML format (easier, documented)
2. **Short-term**: Integrate MPXJ via JNI for MPP reading
3. **Long-term**: Build native Rust implementation incrementally

The most practical approach is to start with alternative formats (MSPDI/MPX) while gradually building native MPP support, or leverage MPXJ through JNI bindings for immediate MPP support.

## References

- [MPXJ Library](https://www.mpxj.org/) - Java library for project file formats
- [Microsoft Q&A: MPP Specification](https://learn.microsoft.com/en-us/answers/questions/cd9dc25c-ac4e-442e-89c3-70bd76f4c9e1/is-there-a-specification-sheet-for-the-mpp-file)
- [MPX Format (Wikipedia)](https://en.wikipedia.org/wiki/MPX_Microsoft_Project_Exchange_File_Format)
- [OLE2 Compound Document Format](https://en.wikipedia.org/wiki/Compound_File_Binary_Format)

