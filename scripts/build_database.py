#/usr/bin/python3
import os
import json
import argparse
import shutil

# Define file extensions for each category
FILE_CATEGORIES = {
    "models": ["obj", "gltf", "glb", "dae", "fbx", "3ds"],
    "images": ["jpeg", "jpg", "png", "bmp", "tiff", "gif", "rgb"],
}

def get_category(extension):
    """Returns the category of a file based on its extension."""
    for category, extensions in FILE_CATEGORIES.items():
        if extension in extensions:
            return category
    return None

def build_file_entry(category, filepath, base_directory, destination_directory):
    """Builds a dictionary entry for a file in the destination directory."""
    relative_path = os.path.relpath(filepath, base_directory)
    destination_path = os.path.join(destination_directory, relative_path)
    name = os.path.splitext(os.path.basename(filepath))[0]
    return {
        "name": name,
        "path": f"./{category}/{os.path.relpath(destination_path, destination_directory)}"
    }

def traverse_and_categorize(source_directory, destination_directory):
    """Traverses the directory and categorizes files."""
    categorized_files = {category: [] for category in FILE_CATEGORIES}
    
    for root, _, files in os.walk(source_directory):
        for file in files:
            extension = file.split(".")[-1].lower()
            category = get_category(extension)
            if category:
                file_path = os.path.join(root, file)
                file_entry = build_file_entry(category, file_path, source_directory, destination_directory)
                categorized_files[category].append(file_entry)
    
    return categorized_files

def write_json_files(data, destination_directory):
    """Writes categorized data to JSON files in the destination directory."""
    for category, files in data.items():
        if files:  # Only write if there are files in the category
            json_path = os.path.join(destination_directory, f"{category}.json")
            with open(json_path, "w") as json_file:
                json.dump(files, json_file, indent=4)
            print(f"Written {category}.json with {len(files)} entries.")

def copy_categorized_files(data, source_directory, destination_directory):
    """Copies categorized files to the destination directory while preserving the directory structure."""
    for category, files in data.items():
        for file in files:
            source_path = os.path.join(source_directory, file["path"].replace(f"./{category}/", ""))
            destination_path = os.path.join(destination_directory, file["path"].replace(f"./", ""))
            os.makedirs(os.path.dirname(destination_path), exist_ok=True)
            shutil.copy2(source_path, destination_path)
            print(f"Copied: {source_path} -> {destination_path}")

def main():
    parser = argparse.ArgumentParser(
        description="Traverse a directory, categorize files, and copy them to a destination directory with JSON summaries."
    )
    parser.add_argument("source_directory", type=str, help="The directory to analyze.")
    parser.add_argument("destination_directory", type=str, help="The directory to store categorized files and JSON summaries.")
    args = parser.parse_args()

    source_directory = args.source_directory
    destination_directory = args.destination_directory

    if not os.path.isdir(source_directory):
        print(f"Error: {source_directory} is not a valid directory.")
        return

    os.makedirs(destination_directory, exist_ok=True)

    categorized_files = traverse_and_categorize(source_directory, destination_directory)
    write_json_files(categorized_files, destination_directory)
    copy_categorized_files(categorized_files, source_directory, destination_directory)

if __name__ == "__main__":
    main()


