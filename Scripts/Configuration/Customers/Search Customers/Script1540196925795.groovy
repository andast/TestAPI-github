import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import java.nio.charset.StandardCharsets

import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import internal.GlobalVariable as GlobalVariable

//def jsonSlurper = new JsonSlurper()

//TestData customers = findTestData('Customers/Customers-1000')
TestData customers = findTestData('Customers/CustomersInternalData')

String str = 'Åstedt'
//String myStrUnicoded = "\u00C5\u0073\u0074\u0065\u0064\u0074"
for (def row : (1..2)) {
    def cust = customers.getObjectValue('lastName', row)
	println(customers.getObjectValue('lastName', row))

	
    responseData = WS.sendRequest(findTestObject('REST API/Calendar/calendar-backend/api/v1/customers/GET search', [('orchestraHost') : GlobalVariable.orchestraHost
                , ('lastName') : cust]))


	println(responseData.getResponseText())
	

    WS.verifyResponseStatusCode(responseData, 200)

   // WS.verifyElementPropertyValue(responseData, "customerList[0].lastName", cust)
}

