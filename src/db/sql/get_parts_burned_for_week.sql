DECLARE @PrevWeekSunday DATETIME
DECLARE @ThisWeekSunday DATETIME
SELECT @PrevWeekSunday = DATEADD(wk, DATEDIFF(wk, 6, GETDATE()), -1)
SELECT @ThisWeekSunday = DATEADD(wk, DATEDIFF(wk, 0, GETDATE()), -1)

SELECT
    REPLACE(PartName, '_', '-') AS Part,
    part.ProgramName AS Program,
    QtyProgram AS Qty,
    NestedArea * QtyProgram AS Area,

    stock.Location,
    stock.PrimeCode AS MaterialMaster,
    NULLIF(stock.Mill,'') AS Wbs,
    
    CASE LEFT(program.MachineName,7)
        WHEN 'Plant_3' THEN 'HS02'
        ELSE 'HS01'
    END AS Plant
FROM PartArchive AS part
    INNER JOIN StockArchive AS stock
        ON part.ArchivePacketID=stock.ArchivePacketID
    INNER JOIN ProgArchive AS program
        ON part.ArchivePacketID=program.ArchivePacketID
        AND program.TransType='SN102'
WHERE part.ArcDateTime >= @PrevWeekSunday
AND part.ArcDateTime < @ThisWeekSunday