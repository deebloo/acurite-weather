pub struct TestWriter {
    pub readings: Vec<acurite::WeatherReading>,
}

#[async_trait::async_trait]
impl acurite::Writer for TestWriter {
    async fn write(&mut self, weather_reading: &acurite::WeatherReading) -> Result<(), ()> {
        self.readings.push(weather_reading.clone());

        Ok(())
    }
}

pub struct ErrorWriter;

#[async_trait::async_trait]
impl acurite::Writer for ErrorWriter {
    async fn write(&mut self, _: &acurite::WeatherReading) -> Result<(), ()> {
        Err(())
    }
}
