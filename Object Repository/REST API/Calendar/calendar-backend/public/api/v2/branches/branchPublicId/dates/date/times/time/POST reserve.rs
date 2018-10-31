<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST reserve</name>
   <tag></tag>
   <elementGuidId>608cc39e-6317-43c0-ab63-75cd0dac690a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;${body}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Referer</name>
      <type>Main</type>
      <value>${orchestraHost}/</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${basicAuth}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${orchestraHost}/calendar-backend/public/api/v2/branches/${branchPublicId}/dates/${date}/times/${time}/reserve</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.orchestraHost</defaultValue>
      <description></description>
      <id>43fbcc0b-47de-4dcb-9571-15021e62f62c</id>
      <masked>false</masked>
      <name>orchestraHost</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.basicAuth</defaultValue>
      <description></description>
      <id>a7d9146c-9d09-48d5-9149-efc732e61883</id>
      <masked>false</masked>
      <name>basicAuth</name>
   </variables>
   <variables>
      <defaultValue>'{}'</defaultValue>
      <description></description>
      <id>ec3951e7-0d73-43db-9a03-08c41e11ed95</id>
      <masked>false</masked>
      <name>body</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e75755ea-fcfd-4ddd-9f78-cb7c320fa2eb</id>
      <masked>false</masked>
      <name>branchPublicId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>yyyy-MM-dd</description>
      <id>82dfa781-c2ea-48b1-92c3-dbe190c5a5fc</id>
      <masked>false</masked>
      <name>date</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>HH:MM</description>
      <id>755eb90d-ceb0-4473-b10c-4dfeeae102a3</id>
      <masked>false</masked>
      <name>time</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
