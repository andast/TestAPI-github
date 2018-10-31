package dataFiles


import static com.kms.katalon.core.testdata.TestDataFactory.findTestData

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.testdata.TestData

public class DataFileUtils {


	/**
	 * Open a test Data File and return ?
	 * @param dataFileName
	 * @return 
	 */

	@Keyword
	def getTestData(String dataFileName){
		TestData customers = findTestData(dataFileName)

		for (def row : (1..customers.getRowNumbers())) {
			println customers.getObjectValue('lastName', row)
		}
		return customers
	}

	/**
	 * Open a test Data File and return testDataId of selected "testParamName"
	 * @param dataFileName
	 * @param testDataName
	 * @return testdataId of the testParamName
	 */

	@Keyword
	def getTestParamId(String dataFileName, String testParamName){
		def data = TestData
		def testParamId = ""
		data = findTestData(dataFileName)

		//String[] colNames = data.getColumnNames()


		for (def row : (1..data.getRowNumbers())) {
			//println data.getValue(1, row)
			if (data.getValue(1, row) == testParamName){
				testParamId = data.getValue(2, row)
			}
		}
		//println testDataId
		//Test global variable
		//GlobalVariable.id = testDataId
		return testParamId
	}


}

