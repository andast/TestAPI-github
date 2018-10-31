<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST reserve</name>
   <tag></tag>
   <elementGuidId>49dc7a32-5464-4c5d-b753-77efdcdfe3a1</elementGuidId>
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
   <restUrl>${orchestraHost}/calendar-backend/public/api/v1/branches/${branchPublicId}/services/${servicePublicId}/dates/${date}/times/${time}/reserve</restUrl>
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
      <id>e55c235e-c2c9-43a6-8f00-e826217ce466</id>
      <masked>false</masked>
      <name>body</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>555be2bc-5f32-4b3b-9865-049e613cecbf</id>
      <masked>false</masked>
      <name>branchPublicId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>71f7482f-63f4-4f0f-990e-65f90ef4f3ae</id>
      <masked>false</masked>
      <name>servicePublicId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>yyyy-MM-dd</description>
      <id>23dbc275-7723-40bb-aa01-974e9bdd7977</id>
      <masked>false</masked>
      <name>date</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>HH:mm</description>
      <id>00959656-3997-4d7a-a1f8-82609cfa581b</id>
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
