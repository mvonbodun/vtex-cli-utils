# VTEX IMPEX Utility (Import / Export)
*vtex_impex* is a command line interface (CLI) to make it easier to import a large volume of catalog data into VTEX.  The CLI uses CSV files as the basis to load data.  In the future it will be expanded to support exporting data.

The goals for this utility:
- Provide a standard layout across CSV files that can be used without the need to know the VTEX internal identifiers
- Be high performant and provide options to use concurrency and threading to speed up the loads

The utility was developed in the RUST language because of it's high performance characteristics, good memory management, and its strong support for building command line utilities.  For more on RUST (https://www.rust-lang.org/).

This utility is most commonly used during an implementation when you need to perform the initial load.  It can also be used for a large batch of changes.  Ongoing updates on a daily basis are better handled by detecting changes in the source system and leveraging an integration tool like Workado, Wevo or Digibee to push the changes in to VTEX in real time.

## How to install

### Download the executable for your OS
1. Download the executables from the [releases](https://github.com/mvonbodun/vtex-cli-utils/releases) page for the project on Github.
2. Make sure to add executable permissions on Mac or Linux
```
chmod +x ./vtex_impex
```
*Note*: On Mac you may get an error when trying to execute the program that it is unsigned software.  You can override this by going to **System Preferences -> Security & Privacy -> General tab** and allowing software from sources other than the App Store

### Build from Source
You can build from source:
- Download [RUST](https://www.rust-lang.org/)
- Checkout the project from Github
- To compile the program - from the root of the **vtex-cli-utils** folder:
```
cargo build
```

### Create a .env file
To run the program, several environment variables need to be set to connect with your VTEX instance.  In the directory where the **vtex_impex** utility was copied, create a .env file
```
ACCOUNT_NAME=
ENVIRONMENT=
VTEX_API_APPKEY=
VTEX_API_APPTOKEN=
```

## How to use the utility
The utility provides command line help.  Open a **Terminal** window on Mac OS X and at the prompt type:
```
vb@michaels-mbp-2 ~ % ./vtex_impex --help
```
This will output the different subcommands
```
VTEX Dataloader 0.2.0
VTEX
Command line interface to import / export data into VTEX

USAGE:
    vtex_impex [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    brand                     actions on the brand into VTEX
    category                  actions on the category into VTEX
    help                      Prints this message or the help of the given subcommand(s)
    inventory                 actions on the inventory into VTEX
    price                     actions on the price into VTEX
    product                   actions on the product into VTEX
    productspecassociation    actions on product specification associations into VTEX
    similarcategory           actions on similarcategory into VTEX
    sku                       actions on the sku into VTEX
    skuean                    actions on skuean into VTEX
    skufile                   actions on skufile (images) into VTEX
    skuspecassociation        actions on sku specification associations into VTEX
    specification             actions for operating on a specification into VTEX
    specificationgroup        actions on the specification group into VTEX
    specificationvalue        actions on the specification value into VTEX```

```
Each sub command has it's own help
```
vb@michaels-mbp-2 ~ % ./vtex_impex specification --help
```
Which provides the following output
```
vtex_impex-specification 0.1.0
actions for operating on a specification into VTEX

USAGE:
    vtex_impex specification [OPTIONS] --action <ACTION> --file <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --action <ACTION>
            The action to perform on the VTEX Object - import, export [possible values: import, genproductspecsfile,
            genskuspecsfile]
    -c, --concurrency <CONCURRENCY>                                      Sets the concurrency value - default is 1
    -f, --file <FILE>
            Sets the input or output file to read or write to.

        --product_file <PRODUCT_FILE>                                    Sets the Product file
        --prod_spec_assigns_file <PRODUCT_SPEC_ASSIGNMENTS_FILE>         Sets the Product Specification Assignments file
        --sku_spec_allowed_values_file <SKU_SPEC_ALLOWED_VALUES_FILE>    Sets the SKU Specification Allowed Values file
```
## Understanding the CSV file formats
Unlike the Google Drive Format Spreadsheet that has been developed by the U.S. 1st Party Apps team, **vtex_impex** uses multiple CSV files to load the data into VTEX.  **vtex_impex** is intended for large datasets (greater than 1000 SKUs) and complex specification requirements.

### The Starting CSV files
The following CSV files are what you should create to use the **vtex_implex** utility to load data:
- [Categories.csv](../data/Categories.csv)
- [Products.csv](../data/Products.csv)
- [Skus.csv](../data/Skus.csv)
- [SpecificationGroups.csv](../data/SpecificationGroups.csv)
- [ProductSpecificationAssignments.csv](../data/ProductSpecificationAssignments.csv)
- [SkuSpecificationAllowedValues.csv](../data/SkuSpecificationAllowedValues.csv)
- [SkuSpecificationValueAssignments.csv](../data/SkuSpecificationValueAssignments.csv)
- [SimilarCategories.csv](../data/SimilarCategories.csv)
- [Prices.csv](../data/Prices.csv)
- [Inventory.csv](../data/Inventory.csv)

The CSV files above alternate identifiers to provide references across the different files since the VTEX Ids are not known in a new load.  Examples:
- In *Categories.csv*, *UniqueIdentifier* and *ParentUniqueIdentifier* allow you to use an alternate reference for a category.  These values carry through into the *Products.csv*
- In *Products.csv*, *RefId* is used in other files like *ProductSpecificationAssignments* and *SkuSpecificationAllowedValues.csv*
- In *Skus.csv*, *RefId* is used in other files like *SkuSpecificationValueAssignments.csv*, *Prices.csv*, *Inventory.csv*

### The Generated CSV files
From the collection of files listed above, the utility will generate other files with VTEX Identifiers resolved so the data can be loaded into VTEX.  Files that are generated:
- [Brands.csv](../data/Brands.csv)
- [ProductSpecifications.csv](../data/ProductSpecifications.csv)
- [ProductSpecificationAssociations.csv](../data/ProductSpecificationAssociations.csv)
- [SkuSpecifications.csv](../data/SkuSpecifications.csv)
- [SpecificationValues.csv](../data/SpecificationValues.csv)
- [SkuSpecificationAssociations.csv](../data/SkuSpecificationAssociations.csv)
- [SkuFiles.csv](../data/SkuFiles.csv)
- [SkuEan.csv](../data/SkuEan.csv)


## Example Loading Data
To better understand how the **vtex_impex** utility works and to understand the CSV formats, we recommend you walk through the following example.  After going through this example, you can begin to change the files with your own data and load it into VTEX.

We recommend you download the [data](../data) folder from the **vtex-cli-utils** project on Github.  Put the data folder in the directory where you put the **vtex_impex** binary.  The folder structure should look like the following:
```
vb@Michaels-MacBook-Pro-2 vtex-cli-utils % pwd
/Users/vb/Software/vtex-cli-utils
vb@Michaels-MacBook-Pro-2 vtex-cli-utils % ls -la
total 57736
drwxr-xr-x   5 vb  staff       160 Mar  7 21:47 .
drwxr-xr-x   9 vb  staff       288 Mar  7 21:44 ..
-rw-r--r--   1 vb  staff       505 Mar  7 21:47 .env
drwxr-xr-x  18 vb  staff       576 Mar  7 21:12 data
-rwxr-xr-x   1 vb  staff  29554744 Mar  7 19:40 vtex_impex
vb@Michaels-MacBook-Pro-2 vtex-cli-utils % 
```
We recommend that you remove the files that will be generated from the data folder [The Generated CSV files](##the-generated-csv-files) before starting the steps below.

*Note*: you will need to use "./" on Mac and Linux in front of "vtex_impex" to run the program if you are in the directory where you copied the program.

To load catalog data into VTEX, the utility supports the following steps right now.  We recommend using the order below for loading the data into VTEX.
- [Category](#category)
- [Brand](#brand)
- [Specification Group](#specification-group)
- [Product Specification](#product-specification)
- [SKU Specification](#sku-specification)
- [Specification Value](#specification-value)
- [Product](#product)
- [SKU](#sku)
- [Product Specification Association](#product-specification-association)
- [SKU Specification Association](#sku-specification-association)
- [SKU File (Images)](#sku-files)
- [SKU EAN](#ean)
- [Similar Categories](#similar-categories)
- [Price](#price)
- [Inventory](#inventory)

## Category
Category uses a CSV layout very similar to the VTEX API for Category with a couple of new columns added:
- UniqueIdentifier - a unique identifier for the Category
- ParentUniqueIdentifier - a reference to the parent category using the UniqueIdentifier

These two fields were introduced to allow you to alias the categories instead of using the Name (which may not be unique across the tree) or having to know the VTEX Category Id

The format looks like the following:
|Id |UniqueIdentifier|Name                      |FatherCategoryId|ParentUniqueIdentifier|Title                     |Description               |Keywords                  |IsActive|LomadeeCampaignCode|AdWordsRemarketingCode|ShowInStoreFront|ShowBrandFilter|ActiveStoreFrontLink|GlobalCategoryId|StockKeepingUnitSelectionMode|Score|LinkId|HasChildren|
|---|----------------|--------------------------|----------------|----------------------|--------------------------|--------------------------|--------------------------|--------|-------------------|----------------------|----------------|---------------|--------------------|----------------|-----------------------------|-----|------|-----------|
|   |1-0-0           |Men                       |                |                      |Men                       |Men                       |Men                       |true    |                   |                      |true            |true           |true                |                |SPECIFICATION                |     |      |           |
|   |1-2-0           |Mens Apparel              |                |1-0-0                 |Mens Apparel              |Mens Apparel              |Mens Apparel              |true    |                   |                      |true            |true           |true                |                |SPECIFICATION                |     |      |           |
|   |1-2-3           |Casual Short Sleeve Shirts|                |1-2-0                 |Casual Short Sleeve Shirts|Casual Short Sleeve Shirts|Casual Short Sleeve Shirts|true    |                   |                      |true            |true           |true                |                |SPECIFICATION                |     |      |           |
|   |1-2-7           |Classic Jeans             |                |1-2-0                 |Classic Jeans             |Classic Jeans             |Classic Jeans             |true    |                   |                      |true            |true           |true                |                |SPECIFICATION                |     |      |           |
|   |1-31-0          |Accessories               |                |1-0-0                 |Accessories               |Mens Accessories          |Accessories               |true    |                   |                      |true            |true           |true                |                |SPECIFICATION                |     |      |           |
|   |1-31-299        |Mens Sunglasses           |                |1-31-0                |Mens Sunglasses           |Mens Sunglasses           |Mens Sunglasses           |true    |                   |                      |true            |true           |true                |                |SPECIFICATION                |     |      |           |

*Note*: A child category cannot be created before it's parent.  Make sure the categories are in the proper sequence of the CSV file.

### Running a category import
To run a category import using the sample data supplied (Note: the terminal command prompt is not shown):
```
RUST_LOG=info ./vtex_impex category -a import -f data/Categories.csv
```
To turn on debugging set *RUST_LOG=debug* Note: You can also set this once in your command line environment variable (bash or .profile depending on your operating system)
```
RUST_LOG=debug ./vtex_impex category -a import -f data/Categories.csv
```
The output should look similar to the following:
```
0.003 [INFO] - Starting data load
0.017 [INFO] - Begin loading categories
0.444 [INFO] - category id: 1: response: 200
0.502 [INFO] - category id: 2: response: 200
0.560 [INFO] - category id: 3: response: 200
0.618 [INFO] - category id: 4: response: 200
0.678 [INFO] - category id: 5: response: 200
0.740 [INFO] - category id: 6: response: 200
0.796 [INFO] - category id: 7: response: 200
0.960 [INFO] - category id: 8: response: 200
1.019 [INFO] - category id: 9: response: 200
1.078 [INFO] - category id: 10: response: 200
1.136 [INFO] - category id: 11: response: 200
1.136 [INFO] - Finished loading categories
1.136 [INFO] - Finished data load
```
*Note*:  If you run the same file again, you will create duplicate categories.  Delete the categories using: [https://{accountName}.myvtex.com/admin/Site/FullCleanUp.aspx](https://{accountName}.myvtex.com/admin/Site/FullCleanUp.aspx)

## Brand
Brand uses a CSV layout very similar to the VTEX API for Category with a couple of new columns added:
- CategoryUniqueIdentifier - a unique identifier for the Category
- BrandName - aThe name of the brand for the product

These two fields were introduced to allow you to alias the categories instead of having to know the Category Id and allowing you to generate the Brand file to load into VTEX.

### Generating the Brand File
If you follow the recommended layout for the Product file, you can generate the Brand file to be loaded into VTEX.  The Product CSV file follows the following format:

|Id |Name                                     |DepartmentId|CategoryId|CategoryUniqueIdentifier|BrandId|BrandName          |LinkId                                              |RefId     |IsVisible|Description                                                                                                                                                                                                                                                         |DescriptionShort|ReleaseDate        |KeyWords                                      |Title                                    |IsActive|TaxCode|MetaTagDescription                       |SupplierId|ShowWithoutStock|AdWordsRemarketingCode|LomadeeCampaignCode|Score|
|---|-----------------------------------------|------------|----------|------------------------|-------|-------------------|----------------------------------------------------|----------|---------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------|-------------------|----------------------------------------------|-----------------------------------------|--------|-------|-----------------------------------------|----------|----------------|----------------------|-------------------|-----|
|   |Columbia Mens Short Sleeve Bonehead Shirt|            |          |1-2-3                   |       |Columbia Sportswear|Columbia-Mens-Short-Sleeve-Bonehead-Shirt-P000007188|P000007188|true     |The Bonehead Shirt is designed for anglers & features a cotton Ultra-lite poplin fabric for a lived-in comfort, hook & loop-closed fly boxed pockets, rod holder, utility loop, tool holder & is fully vented.  International Shipping Prohibited.                  |                |2021-10-01T00:00:00|Columbia, Mens, Short, Sleeve, Bonehead, Shirt|Columbia Mens Short Sleeve Bonehead Shirt|true    |       |Columbia Mens Short Sleeve Bonehead Shirt|          |true            |                      |                   |     |
|   |Columbia Sportswear Mens Bahama II Shirt |            |          |1-2-3                   |       |Columbia Sportswear|Columbia-Sportswear-Mens-Bahama-II-Shirt-P000039567 |P000039567|true     |A lightweight shirt that dries fast & has mesh-lined vents at the back enhance airflow to keep you cool & features built-in UV protection . This Bahama II shirt offers two chest pockets offering plenty of room for small gear. International Shipping Prohibited.|                |2021-10-01T00:00:00|Columbia, Sportswear, Mens, Bahama, II, Shirt |Columbia Sportswear Mens Bahama II Shirt |true    |       |Columbia Sportswear Mens Bahama II Shirt |          |true            |                      |                   |     |

The program will loop through all the records in the Product file and extract the brand names.  It will not create duplicates.

To generate a brand file:
```
RUST_LOG=info ./vtex_impex brand -a genbrandfile -f Brands.csv --product_file data/Products.csv
```
It should generate output like the following:
```
0.005 [INFO] - Starting data load
0.041 [INFO] - Wrote 60 brand records
0.041 [INFO] - Finished data load
```

### Running a Brand import
To run a brand import:
```
RUST_LOG=info ./vtex_impex brand -a import -f data/Brands.csv
```
You should see output like the following:
```
0.004 [INFO] - Starting data load
0.019 [INFO] - brand records: 60
0.452 [INFO] - brand: None: response: 200
0.513 [INFO] - brand: None: response: 200
...
4.096 [INFO] - brand: None: response: 200
4.096 [INFO] - Finished loading brands
4.097 [INFO] - Finished data load
```

## Specification Group
Specification Group uses a CSV file that matches the VTEX API exactly.  Most people recommend creating just one specification group as this is deprecated functionality from VTEX Classic CMS.

The format for the CSV file:
|Id |Name                       |CategoryId|Position|
|---|---------------------------|----------|--------|
|   |Default Specification Group|          |        |

*Note*: The CategoryId is left blank.  The group will be created at the root category.

**Default Specification Group** is a hard coded value.  Do not change this.  This will be changed in the future to be more flexible.


### Running a Specification Group import
To run a specification group import:
```
RUST_LOG=info ./vtex_impex specificationgroup -a import -f data/SpecificationGroups.csv
```
You should see output like the following:
```
0.000 [INFO] - Starting data load
0.003 [INFO] - Starting specification group load
0.004 [INFO] - specification group records: 1
0.451 [INFO] - specification group: None: repsonse: 200
0.452 [INFO] - Finished loading specification groups
0.452 [INFO] - Finished data load
```

## Product Specification
Product Specification uses a CSV format that allows for the generation of the appriate VTEX files.  The format has a row for each specification and its value by product.  The file relies on the ProductRefId field.

The format looks like the following:
|ProductRefId|Name     |Description|Value                                              |Position|IsFilter|IsRequired|IsOnProductDetails|IsStockKeepingUnit|IsWizard|IsActive|IsTopMenuLinkActive|DefaultValue|
|------------|---------|-----------|---------------------------------------------------|--------|--------|----------|------------------|------------------|--------|--------|-------------------|------------|
|P000007188  |SizeChart|SizeChart  |http://www.beallsflorida.com/default?page=SizeChart|0       |false   |false     |false             |false             |false   |true    |false              |            |
|P000007188  |Material |Material   |Cotton                                             |0       |false   |false     |false             |false             |false   |true    |false              |            |

### Generating the Product Specification file
The program will generate the specifications and add them to the leaf level category for the product.  *Note*: The values for the specification are assigned later.  To do this, multiple files are needed to determine the parent categories since the aliases are being used:
- -f &emsp; is the output file
- --prod_spec_assigns_file &emsp; has the specications and values for each product
- --product_file &emsp; shows the products parent category via the ParentUniqueIdentifier

To generate the file:
```
RUST_LOG=info ./vtex_impex specification -a genproductspecsfile -f data/ProductSpecifications.csv --prod_spec_assigns_file data/ProductSpecificationAssignments.csv --product_file transform/data/out/Products.csv
```
You should see output like this:
```
0.001 [INFO] - Starting data load
0.004 [INFO] - Starting product specification file generation
1.443 [INFO] - Finished generating product specification file
1.443 [INFO] - finished generating product specifications file
1.443 [INFO] - Finished data load
```
*Note*: The action **genproductspecsfile** creates all the specifications as a **FieldTypeId** of "1" which is "Text".  If you want a defined set of values for the Specification, and used a FieldTypeId of "6" which is "Radio Button" you will need to handle this manually.  This should only matter if you plan to use the VTEX Admin interface to create new products and want to be able to select a value rather than manually enter the text.

### Running a Product Specification Import
To run a product specification import:
```
RUST_LOG=info ./vtex_impex specification -a import -f data/ProductSpecifications.csv
```
You should see output like the following:
```
0.001 [INFO] - Starting data load
0.004 [INFO] - Starting specification load
0.013 [INFO] - specification records: 523
0.408 [INFO] - specification : None: repsonse: 200
0.491 [INFO] - specification : None: repsonse: 200
0.556 [INFO] - specification : None: repsonse: 200
...
36.413 [INFO] - specification : None: repsonse: 200
36.482 [INFO] - specification : None: repsonse: 200
36.483 [INFO] - Finished loading specifications
36.483 [INFO] - finished loading specifications
36.483 [INFO] - Finished data load
```

## SKU Specification
SKU Specification uses a CSV format that allows for the generation of the appriate VTEX files.  The format uses a row for each product, the attribute name and then its allowed values for that attribute to the right.

The format looks like the following:
|ProductRefId|Name |Position|AllowedValue1  |AllowedValue2  |AllowedValue3|AllowedValue4|AllowedValue5|AllowedValue6|AllowedValue7|
|------------|-----|--------|---------------|---------------|-------------|-------------|-------------|-------------|-------------|
|P000164920  |Color|1       |SANDED BRO BLUE|FADED LITE BLUE|             |             |             |             |             |
|P000170879  |Color|1       |YELLOW         |PLACID BLUE    |BEIGE        |WHITE        |             |             |             |
|P000007188  |Size |1       |Sm             |Md             |Lg           |X-Lg         |XX-Lg        |             |             |
|P000164920  |Size |1       |32W 30L        |32W 32L        |33W 32L      |34W 29L      |34W 30L      |34W 32L      |34W 34L      |


*Note*: The ProductRefId is used, the specification is the Name Column - in this case "Color" and the allowed values are shown to the right.

### Generating the SKU Specification File
The program will generate the specifications and add them to the leaf level category for the product.  *Note*: The values for the specification are assigned later.  To do this, multiple files are needed to determine the parent categories since the aliases are being used.
- -f &emsp; is the output file
- --sku_spec_allowed_values_file &emsp; has the specications and values for each product that can be set at the SKU Level
- --product_file &emsp; shows the products parent category via the ParentUniqueIdentifier


To generate the file:
```
RUST_LOG=info ./vtex_impex specification -a genskuspecsfile -f data/SkuSpecifications.csv --sku_spec_allowed_values_file data/SkuSpecificationAllowedValues.csv --product_file data/Products.csv
```
You should see output like this:
```
0.001 [INFO] - Starting data load
0.005 [INFO] - Starting SKU specification file generation
1.531 [INFO] - Finished generating SKU specification file
1.531 [INFO] - finished generating sku specifications file
1.531 [INFO] - Finished data load
```
*Note*:  The action **genskuspecsfile** generates a file with FieldTypeId always set to "6" which is "Radio Button".  SKU Specificaitons have a defined set of values.  If you want to use a different FieldTypeId, you would need to manually change it.

### Running a SKU Specification Import
To run a SKU specification import:
```
RUST_LOG=info ./vtex_impex specification -a import -f data/SkuSpecifications.csv
```
You should see output like the following:
```
0.001 [INFO] - Starting data load
0.006 [INFO] - Starting specification load
0.007 [INFO] - specification records: 10
0.447 [INFO] - specification : None: repsonse: 200
0.531 [INFO] - specification : None: repsonse: 200
0.620 [INFO] - specification : None: repsonse: 200
...
1.116 [INFO] - specification : None: repsonse: 200
1.178 [INFO] - specification : None: repsonse: 200
1.179 [INFO] - Finished loading specifications
1.179 [INFO] - Finished data load
```

## Specification Value
Specification Values are generated based on two existing files:
- sku_spec_allowed_values_file
- product_file

The program will ensure that the values remain unique even if specification values are replicated across products

### To generate a Specification Value File
To generate a Specification Value file:
```
RUST_LOG=info ./vtex_impex specificationvalue -a genspecvaluesfile -f data/SpecificationValues.csv --product_file data/Products.csv --sku_spec_allowed_values_file data/SkuSpecificationAllowedValues.csv
```
You should see output like the following:
```
0.001 [INFO] - Starting data load
0.004 [INFO] - Starting generation of specification values file
1.843 [INFO] - Finished specification values file generation
1.844 [INFO] - Finished data load
```

### Running a Specification Value Import
To run a Specification Value import:
```
RUST_LOG=info ./vtex_impex specificationvalue -a import -f data/SpecificationValues.csv
```
You should see output like the following:
```
0.004 [INFO] - Starting data load
0.017 [INFO] - Starting specification values load
0.395 [INFO] - name: "801 BR PCH": response: 200
0.458 [INFO] - name: "LEMON YELLOW": response: 200
0.516 [INFO] - name: "GYPSY RED": response: 200
...
32.761 [INFO] - name: "30": response: 200
32.841 [INFO] - name: "32": response: 200
32.843 [INFO] - finished loading specification values
32.843 [INFO] - Finished data load
```

## Product
The CSV file used to load products follows the VTEX API but has two additional fields:
- CategoryUniqueIdentifier - the link to the category the product belongs to
- BrandName - the name of the brand (used to generate the Brand File)

The format of the file looks like the following:
|Id |Name                                     |DepartmentId|CategoryId|CategoryUniqueIdentifier|BrandId|BrandName          |LinkId                                              |RefId     |IsVisible|Description                                                                                                    |DescriptionShort|ReleaseDate        |KeyWords                                      |Title                                    |IsActive|TaxCode|MetaTagDescription                       |SupplierId|ShowWithoutStock|AdWordsRemarketingCode|LomadeeCampaignCode|Score|
|---|-----------------------------------------|------------|----------|------------------------|-------|-------------------|----------------------------------------------------|----------|---------|---------------------------------------------------------------------------------------------------------------|----------------|-------------------|----------------------------------------------|-----------------------------------------|--------|-------|-----------------------------------------|----------|----------------|----------------------|-------------------|-----|
|   |Columbia Mens Short Sleeve Bonehead Shirt|            |          |1-2-3                   |       |Columbia Sportswear|Columbia-Mens-Short-Sleeve-Bonehead-Shirt-P000007188|P000007188|true     |The Bonehead Shirt is designed for anglers & features a cotton Ultra-lite poplin fabric for a lived-in comfort.|                |2021-10-01T00:00:00|Columbia, Mens, Short, Sleeve, Bonehead, Shirt|Columbia Mens Short Sleeve Bonehead Shirt|true    |       |Columbia Mens Short Sleeve Bonehead Shirt|          |true            |                      |                   |     |
|   |Columbia Sportswear Mens Bahama II Shirt |            |          |1-2-3                   |       |Columbia Sportswear|Columbia-Sportswear-Mens-Bahama-II-Shirt-P000039567 |P000039567|true     |A lightweight shirt that dries fast & has mesh-lined vents at the back.                                        |                |2021-10-01T00:00:00|Columbia, Sportswear, Mens, Bahama, II, Shirt |Columbia Sportswear Mens Bahama II Shirt |true    |       |Columbia Sportswear Mens Bahama II Shirt |          |true            |                      |                   |     |

### Running a Product Import
To run a product import:
```
RUST_LOG=info ./vtex_impex product -a import -f data/Products.csv
```
You should see output like the following:
```
0.001 [INFO] - Starting data load
0.005 [INFO] - Starting load of products
1.553 [INFO] - product: Some("P000007188"): response: 200
1.644 [INFO] - product: Some("P000039567"): response: 200
...
34.469 [INFO] - product: Some("P000226227"): response: 200
34.546 [INFO] - product: Some("P000226229"): response: 200
34.546 [INFO] - finished loading products
34.546 [INFO] - Finished data load
```

## SKU
The CSV file to load SKUs follows the VTEX API but has two additonal columns:
- ProductRefId - used instead of ProductId
- ImageUrl - the image of the product

The format looks like the following:
|Id |ProductId|ProductRefId|IsActive|Name                   |RefId   |ImageUrl                                                                                                               |PackagedHeight|PackagedLength|PackagedWidth|PackagedWeightKg|Height|Length|Width|WeightKg|CubicWeight|IsKit|CreationDate       |RewardValue|EstimatedDateArrival|ManufacturerCode|CommercialConditionId|MeasurementUnit|UnitMultiplier|ModalType|KitItensSellApart|ActivateIfPossible|EAN       |
|---|---------|------------|--------|-----------------------|--------|-----------------------------------------------------------------------------------------------------------------------|--------------|--------------|-------------|----------------|------|------|-----|--------|-----------|-----|-------------------|-----------|--------------------|----------------|---------------------|---------------|--------------|---------|-----------------|------------------|----------|
|   |         |P000206871  |false   |Color GREEN TEA Size Lg|94124836|https://images.beallsflorida.com/i/beallsflorida/457-1007-3332-31-yyy?w=1000&h=1000&fmt=auto&qlt=default&img404=404&v=1|0             |0             |0            |0               |0     |0     |0    |0       |0          |false|2021-10-01T00:00:00|           |                    |                |                     |un             |1             |         |false            |true              |1212121   |
|   |         |P000214882  |false   |Color WHITE Size X-Lg  |95397041|https://images.beallsflorida.com/i/beallsflorida/457-4896-2034-10-yyy?w=1000&h=1000&fmt=auto&qlt=default&img404=404&v=1|0             |0             |0            |0               |0     |0     |0    |0       |0          |false|2021-10-01T00:00:00|           |                    |                |                     |un             |1             |         |false            |true              |3434343   |

### Running a SKU Import
To run a SKU Import:
```
RUST_LOG=info ./vtex_impex sku -a import -f data/Skus.csv
```
You should see output like the following:
```
0.000 [INFO] - Starting data load
0.005 [INFO] - Starting SKU load
25.277 [INFO] - sku: "94124836": response: 200
25.356 [INFO] - sku: "95397041": response: 200
25.434 [INFO] - sku: "96213270": response: 200
...
50.774 [INFO] - sku: "81688926": response: 200
250.835 [INFO] - sku: "93462311": response: 200
250.835 [INFO] - finished SKU load
250.836 [INFO] - Finished data load
```

## Product Specification Association
The Product Specification Association file is generated from two files:
- prod_spec_assigns_file - was generated in a previous step above (TODO: Add link)
- product_file - used to get the product parent category

### To generate a Product Specification Association File
To generate the file:
```
RUST_LOG=info ./vtex_impex productspecassociation -a genproductspecassocfile -f data/ProductSpecificationAssociations.csv --prod_spec_assigns_file data/ProductSpecificationAssignments.csv --product_file data/Products.csv
```
You should see output like the following:
```
0.001 [INFO] - Starting data load
0.005 [INFO] - Starting generate product spec assoocation file
26.862 [INFO] - Finished generating Product Spec Association file
26.862 [INFO] - Finished data load
```

### Running a Product Specification Association Import
To run a Product Specification Association Import:
```
RUST_LOG=info ./vtex_impex productspecassociation -a import -f data/ProductSpecificationAssociations.csv
```
You should see output like the following:
```
0.003 [INFO] - Starting data load
0.019 [INFO] - Starting product spec association load
0.432 [INFO] - product: 1  text: Some("http://www.beallsflorida.com/default?page=SizeChart"):  response: 200
0.496 [INFO] - product: 124  text: Some("http://www.beallsflorida.com/default?page=MnsPants"):  response: 200
0.
...
353.361 [INFO] - product: 331  text: Some("Relaxed"):  response: 200
353.419 [INFO] - product: 331  text: Some("12 inseam"):  response: 200
353.481 [INFO] - product: 331  text: Some("Tab-Front"):  response: 200
353.481 [INFO] - finished product spec association load
353.481 [INFO] - Finished data load
```

## SKU Specification Association
The SKU Specification Association file is generated from three files:
- sku_spec_assigns_file - the file with the specification assignments
- product_file - used to determine the parent category
- sku_file - used to determine the parent product

### To generate a SKU Specification Association File
To generate a SKU Specification Association file:
```
RUST_LOG=info ./vtex_impex skuspecassociation -a genskuspecassocfile -f data/SkuSpecificationAssociations.csv --sku_spec_assigns_file data/SkuSpecificationValueAssignments.csv --product_file data/Products.csv --sku_file data/Skus.csv
```
You should see output like the following:
```
0.003 [INFO] - Starting data load
0.015 [INFO] - Staring generation of SKU Spec Association file
0.015 [INFO] - Start creating sku_id_lookup
0.015 [INFO] - Start get_all_sku_ids()
0.522 [INFO] - Finished get_all_sku_ids: 3369 records in 506.939666ms
0.523 [INFO] - Starting get_item_records()
16.212 [INFO] - finished get_item_records(): item_recs.len(): 3369
16.247 [INFO] - Finish creating sku_id_lookup length: 3369
45.867 [INFO] - records written: 6730
45.867 [INFO] - Finished generating SKU Spec Association file
45.868 [INFO] - Finished data load
```

### Running a SKU Specification Association Import
To run a SKU Specification Association Import
```
 RUST_LOG=info ./vtex_impex skuspecassociation -a import -f data/SkuSpecificationAssociations.csv
```
You should see output like the following:
```
0.003 [INFO] - Starting data load
0.013 [INFO] - Starting load of SKU Spec Associations
0.560 [INFO] - product: 402  text: None:  response: 200
0.638 [INFO] - product: 351  text: None:  response: 200
...
465.799 [INFO] - product: 693  text: None:  response: 200
465.863 [INFO] - product: 664  text: None:  response: 200
465.928 [INFO] - product: 2463  text: None:  response: 200
465.928 [INFO] - finished load of SKU Spec Associations
465.928 [INFO] - Finished data load
```

## SKU Files
The SKU Files CSV is generated from the Skus.csv file format which contains an ImageUrl column that points to the image for the SKU.

### To generate the SKU Files file
To generate the SKU Files file:
```
RUST_LOG=info ./vtex_impex skufile -a genskufile -f data/SkuFiles.csv --sku_file data/Skus.csv
```
You should see output like the following:
```
0.001 [INFO] - Starting data load
0.005 [INFO] - Starting generation of SKU Files file
0.006 [INFO] - Start creating sku_id_lookup
0.006 [INFO] - Start get_all_sku_ids()
0.629 [INFO] - Finished get_all_sku_ids: 3369 records in 623.006666ms
0.629 [INFO] - Starting get_item_records()
19.435 [INFO] - finished get_item_records(): item_recs.len(): 3369
19.469 [INFO] - Finish creating sku_id_lookup length: 3369
19.573 [INFO] - records writtern: 3369
19.573 [INFO] - Finished generating SKU Files file
19.574 [INFO] - Finished data load
```

### Running a SKU Files import
To run a SKU File import:
```
RUST_LOG=info ./vtex_impex skufile -a import -f data/SkuFiles.csv
```
You should see output like the following:
```
0.004 [INFO] - Starting data load
0.016 [INFO] - Starting load of SKU Files file
1.110 [INFO] - sku_id: 1  image: Some("https://images.beallsflorida.com/i/beallsflorida/457-1007-3332-31-yyy?w=1000&h=1000&fmt=auto&qlt=default&img404=404&v=1"):  response: 200
1.902 [INFO] - sku_id: 2  image: Some("https://images.beallsflorida.com/i/beallsflorida/457-4896-2034-10-yyy?w=1000&h=1000&fmt=auto&qlt=default&img404=404&v=1"):  response: 200
...
653.536 [INFO] - sku_id: 3368  image: Some("https://images.beallsflorida.com/i/beallsflorida/479-2296-0031-98-yyy?w=1000&h=1000&fmt=auto&qlt=default&img404=404&v=1"):  response: 200
653.682 [INFO] - sku_id: 3369  image: Some("https://images.beallsflorida.com/i/beallsflorida/110-7171-2095-31-yyy?w=1000&h=1000&fmt=auto&qlt=default&img404=404&v=1"):  response: 200
653.683 [INFO] - finished loading SKU Files file
653.683 [INFO] - Finished data load
```

## EAN
The EAN File CSV is generated from the Skus.csv file format which contains an EAN column that has the EAN for the SKU.  The EAN column is optional.

### To generate the SKU EAN file
To generate the EAN file:
```
RUST_LOG=info ./vtex_impex skuean -a genskueanfile -f data/SkuEan.csv --sku_file data/Skus.csv
```
You should see out put like the following:
```
0.003 [INFO] - Starting data load
0.020 [INFO] - Starting generation of SKU EAN file
0.020 [INFO] - Start creating sku_id_lookup
0.020 [INFO] - Start get_all_sku_ids()
0.708 [INFO] - Finished get_all_sku_ids: 3369 records in 687.075916ms
0.708 [INFO] - Starting get_item_records()
18.734 [INFO] - finished get_item_records(): item_recs.len(): 3369
18.772 [INFO] - Finish creating sku_id_lookup length: 3369
18.774 [INFO] - records writtern: 3
18.774 [INFO] - Finished generating SKU EAN file
18.774 [INFO] - Finished data load
```

### Running an EAN File import
To run an EAN File import:
```
RUST_LOG=info ./vtex_impex skuean -a import -f data/SkuEan.csv
```
You should see output like the following:
```
0.003 [INFO] - Starting data load
0.015 [INFO] - Starting load of SKU EAN file
0.437 [INFO] - sku_id: 1  ean: "12121212":  response: 200
0.606 [INFO] - sku_id: 3  ean: "34343434":  response: 200
0.753 [INFO] - sku_id: 5  ean: "56565656":  response: 200
0.754 [INFO] - finished loading SKU EAN file
0.754 [INFO] - Finished data load
```

## Similar Categories
The Similar Categories CSV follows the VTEX API.  There are only two columns:
- ProductId - this is the VTEX Id of the Product
- CategoryId - this is the VTEX id of the Category

The file is in the following format:
|ProductId|CategoryId|
|---------|----------|
|1        |253       |
|1        |347       |
|2        |102       |

### Running a Similar Categories import
The Similar Categories API does not appear to be rate limited.  You may want to experiment with the CONCURRENCY VALUE: **-c**

To run a Similar Categories import (this example sets teh concurrency parameter to 12):
```
RUST_LOG=debug ./vtex_impex similarcategory -a import -f data/SimilarCategories.csv
```
You should see output like the following:
```
0.000 [INFO] - Starting data load
0.013 [INFO] - Starting Similar Categories load
0.017 [INFO] - 3 records read from input file: data/SimilarCategories.csv
0.624 [INFO] - product: 1: category: 4: response: 200
0.703 [INFO] - product: 1: category: 6: response: 200
0.799 [INFO] - product: 2: category: 11: response: 200
0.799 [INFO] - finished Similar Categories load
0.799 [INFO] - Finished data load
```

## Price
The Price CSV follows the VTEX API.  It only handles basic pricing today - not price lists.  There is one additional column added:
- refId - this is the SKU refId and is used to lookup the SKU Id

The file is in the following format:
|skuId|refId   |markup|listPrice|basePrice|costPrice|
|-----|--------|------|---------|---------|---------|
|     |32448426|      |34.99    |34.99    |34.99    |
|     |32448453|      |34.99    |34.99    |34.99    |
|     |32448478|      |34.99    |34.99    |34.99    |
|     |32448480|      |40       |40       |40       |
|     |32448508|      |34.99    |34.99    |34.99    |

### Running a Price import
The Price API at VTEX is rate limited to 40 inserts/updates a second.  The **-r** parameter is used to set the rate limit.  In our testing, 36 worked best.  The **-c** parameter is used to set the CONCURRENCY.

To run a Price import:
```
RUST_LOG=info ./vtex_impex price -a import -f data/Prices.csv -c 4 -r 36
```
You should see the following output:
```
0.003 [INFO] - Starting data load
0.015 [INFO] - Start creating sku_id_lookup
0.015 [INFO] - Start get_all_sku_ids()
0.666 [INFO] - Finished get_all_sku_ids: 3369 records in 650.42125ms
0.666 [INFO] - Starting get_item_records()
25.975 [INFO] - finished get_item_records(): item_recs.len(): 3369
26.012 [INFO] - Finish creating sku_id_lookup length: 3369
26.374 [INFO] - sku: Some(1475): response: 200
26.396 [INFO] - sku: Some(129): response: 200
...
120.430 [INFO] - sku: Some(464): response: 200
120.432 [INFO] - sku: Some(2208): response: 200
120.432 [INFO] - finished load_prices
120.434 [INFO] - finished loading prices
120.434 [INFO] - Finished data load
```

## Inventory
The Inventory CSV follows the VTEX API.  There is one additional column added:
- refId - this is the SKU refId and is used to lookup the SKU Id

The file is in the following format:
|warehouseId|skuId|refId   |unlimitedQuantity|dateUtcOnBalanceSystem|quantity|
|-----------|-----|--------|-----------------|----------------------|--------|
|warehouse1 |     |32448426|false            |                      |1       |
|warehouse1 |     |32448453|false            |                      |1       |
|warehouse1 |     |32448478|false            |                      |1       |
|warehouse1 |     |32448480|false            |                      |1       |

### Running an Inventory import
The Inventory API does not appear to be rate limited.  You may want to experiment with the CONCURRENCY VALUE: **-c**

To run an Inventory import (this example sets the concurrency parameter to 12):
```
RUST_LOG=info ./vtex_impex inventory -a import -f data/Inventory.csv -c 12
```
You should see the following output:
```
0.001 [INFO] - Starting data load
0.006 [INFO] - Starting load of Inventory
0.006 [INFO] - Start creating sku_id_lookup
0.006 [INFO] - Start get_all_sku_ids()
0.641 [INFO] - Finished get_all_sku_ids: 3369 records in 634.383583ms
0.641 [INFO] - Starting get_item_records()
20.307 [INFO] - finished get_item_records(): item_recs.len(): 3369
20.343 [INFO] - Finish creating sku_id_lookup length: 3369
20.360 [INFO] - inventory records: 3369
20.449 [INFO] - sku: Some(568): response: 200
20.458 [INFO] - sku: Some(731): response: 200
...
39.215 [INFO] - sku: Some(464): response: 200
39.216 [INFO] - sku: Some(2208): response: 200
39.216 [INFO] - finished loading inventory
39.218 [INFO] - Finished data load
```
