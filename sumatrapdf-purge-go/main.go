package main

import (
	"bytes"
	"errors"
	"fmt"
	"os"
)

func main() {
	const fileStatesStartSeparator = "FileStates"
	const fileStatesStartOffset = 13 // 14 on windows due to \r\n
	const fileStatesEndSeparator = "\n]"
	const fileStateEndSeparator = "\n\t]\n"

	// read data to file
	data, err := os.ReadFile("settings.txt")
	if err != nil {
		fmt.Println("Not able to read file {settings.txt}")
		return
	}

	start_idx := bytes.Index(data, []byte(fileStatesStartSeparator))
	if start_idx == -1 {
		fmt.Println("Not able to find the FileStates block, aborting.")
		return
	}
	start_idx += fileStatesStartOffset

	delta_idx := bytes.Index(data[start_idx:], []byte(fileStatesEndSeparator))
	if delta_idx == -1 {
		fmt.Println("Not able to find the end of the FileStates block, aborting.")
		return
	}

	end_idx := start_idx + delta_idx

	filestates := data[start_idx:end_idx]
	filestate_slice := bytes.SplitN(filestates, []byte(fileStateEndSeparator), -1)

	fmt.Printf("Parsed %v entries\n", len(filestate_slice))

	existing_filestates := make([][]byte, 0, len(filestate_slice))

	for _, filestate := range filestate_slice {
		filepath, err := getFilepath(filestate)
		if err != nil {
			fmt.Println("Failed to parse filepath, aborting.")
			return
		}
		filestat, err := os.Stat(filepath)

		if filestat == nil {
			continue
		}

		// filepath exists
		filestate := append(filestate, []byte(fileStateEndSeparator)...)
		existing_filestates = append(existing_filestates, filestate)

	}

	// fmt.Printf("%v", string(data[:start_idx]))
	// for _, existing_filestate := range existing_filestates {
	// 	fmt.Printf("%v", string(existing_filestate))
	// }
	// fmt.Printf("%v", string(data[end_idx+1:])) // +1 avoids double counting a \n

	err = os.Rename("settings.txt", "settings.bak")
	if err != nil {
		fmt.Println("Could not create backup file, aborting.")
	}
	// create new file in append mode
	file, err := os.OpenFile("settings.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		fmt.Println("Could not create new settings file, aborting")
		return
	}
	defer file.Close()

	file.Write(data[:start_idx])
	for _, filestate := range existing_filestates {
		file.Write(filestate)
	}
	file.Write(data[end_idx+1:])

	fmt.Println("done.")

}

func getFilepath(filestate []byte) (string, error) {
	filepath_start_idx := len("\t[\n\t\tFilePath = ")
	delta_idx := bytes.Index(filestate[filepath_start_idx:], []byte("\n"))
	if delta_idx == -1 {
		fmt.Println()
		return "", errors.New("Error parsing filepath, aborting.")

	}
	filepath_end_idx := filepath_start_idx + delta_idx

	filepath := string(filestate[filepath_start_idx:filepath_end_idx])

	return filepath, nil

}

func exists(filepath []byte) bool {
	return true
}
